//! Who controls an object: the derived controller every rules-facing read
//! uses, and the one rule a change of that controller goes through.
//!
//! `GameObject.controller` is the base controller: what the object entered
//! under [CR#110.2], moved by a one-shot `ControlChanged`. A continuous
//! control-changing effect applies in layer 2 [CR#613.1b] and changes only the
//! layered view's controller, so the rules read that one.

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameState;

impl GameState {
    /// The controller of `id` as the rules see it: a card-backed object's
    /// derived controller, after layer-2 control-changing effects
    /// ([CR#613.1b]), and a player proxy's own player.
    ///
    /// Builds a fresh layered view; a caller that reads many controllers in
    /// one step or scan builds one view and reads `view.controller(id)`.
    ///
    /// Inside a layer pass this answers with the BASE controller, since
    /// deriving the controller there would start the pass again. The pass
    /// itself reads the derived controller from its `working` map, but these
    /// reads reach `controller_of` from inside it and so see the base one, a
    /// stolen permanent's owner included:
    /// - the frameless matcher (`target::matches_with`), which a layer
    ///   condition delegates player proxies to: its `ControlledBy`, `Controls`
    ///   and `ControllerOf` reads, and the carrier frame its `Where` arm and
    ///   count resolution build;
    /// - `controller_of_source`, the carrier's `You` when that matcher resolves
    ///   a controller register (its `Ref(Reg)` arm and
    ///   `resolve_carrier_register`).
    ///
    /// # Panics
    ///
    /// Panics on a stale `ObjectId` — an engine invariant, not caller input.
    #[must_use]
    pub fn controller_of(&self, id: ObjectId) -> PlayerId {
        match self.objects.obj(id).source {
            ObjectSource::Player(p) => p,
            // The layer system's own input (see above).
            ObjectSource::Card(_) if crate::layer::deriving() => self.objects.obj(id).controller,
            ObjectSource::Card(_) => self.layers().controller(id),
        }
    }

    /// [CR#302.6]: a permanent whose derived controller differs from the one
    /// last observed has changed control, so its new controller has not
    /// controlled it continuously since their most recent turn began: it is
    /// summoning sick until that player's next turn begins. This is the one
    /// rule both control-change paths go through: a one-shot `ControlChanged`
    /// moves the base controller, a layer-2 continuous effect ([CR#613.1b])
    /// moves only the derived one, and either way the derived controller is
    /// what this compares.
    ///
    /// Runs where control can have changed: at the start of every step (so a
    /// continuous effect that began, ended or changed its affected set since
    /// is seen before anything reads sickness or the turn begins) and inside
    /// the `ControlChanged` apply.
    pub(crate) fn observe_control_changes(&mut self) {
        let view = self.layers();
        let battlefield = self.zones.battlefield.clone();
        for id in battlefield {
            let now = view.controller(id);
            let object = self.objects.obj_mut(id);
            if object.observed_controller != now {
                object.observed_controller = now;
                object.summoning_sick = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sylex_card::Card;
    use sylex_card::CardFace;
    use sylex_card::Characteristics;
    use sylex_core::Ability;
    use sylex_core::Cmp;
    use sylex_core::Condition;
    use sylex_core::Count;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::PlayerAttr;
    use sylex_core::Reference;
    use sylex_core::StatValue;
    use sylex_core::StaticSpec;
    use sylex_core::Type;
    use sylex_core::Zone;

    use crate::layer::ContinuousEffect;
    use crate::layer::ScopeResolved;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::object::Timestamp;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    const P0: PlayerId = PlayerId(0);
    const P1: PlayerId = PlayerId(1);

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(P0),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// A permanent `owner` controls on the battlefield, printing `types`,
    /// `power`/`toughness` and `abilities`.
    fn permanent(
        state: &mut GameState,
        owner: PlayerId,
        name: &str,
        types: Vec<Type>,
        power_toughness: Option<i32>,
        abilities: Vec<Ability>,
    ) -> ObjectId {
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
            power: power_toughness.map(StatValue::Number),
            toughness: power_toughness.map(StatValue::Number),
            abilities,
            ..Characteristics::default()
        }));
        let card = state.cards.push(Arc::new(card), owner);
        let id = state
            .objects
            .mint(ObjectSource::Card(card), owner, Some(Zone::Battlefield));
        state.zones.battlefield.push(id);
        id
    }

    /// `to` gains control of `id` through a locked layer-2 `SetController`
    /// with timestamp `at` (Avarice Totem's continuous half).
    fn steal(state: &mut GameState, id: ObjectId, to: PlayerId, at: Timestamp) {
        state.continuous.push(ContinuousEffect {
            timestamp: at,
            controller: to,
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::SetController(
                Reference::controller_parameter(),
            )],
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
    }

    /// [CR#109.5]: a conditional static ability's "you" is the current
    /// controller of the permanent it is on. Serra Ascendant ("As long as you
    /// have 30 or more life, this creature gets +5/+5 and has flying.", its
    /// pump half) belongs to P0 at 20 life; P1, at 30 life, steals it, and
    /// the condition now reads P1's life.
    #[test]
    fn stolen_conditional_static_reads_its_new_controllers_life() {
        let mut state = game();
        let ascendant = permanent(
            &mut state,
            P0,
            "Serra Ascendant",
            vec![Type::Creature],
            Some(1),
            vec![Ability::r#static(StaticSpec::Conditionally(
                Condition::Compare(
                    Count::PlayerStatOf(Reference::controller_parameter(), PlayerAttr::Life),
                    Cmp::AtLeast,
                    Count::Literal(30),
                ),
                Arc::new(StaticSpec::Modify(
                    Reference::source_parameter(),
                    Modification::Power(NumericOp::Up(Count::Literal(5))),
                )),
            ))],
        );
        state.player_mut(P1).life = 30;
        assert_eq!(
            state.layers().power(ascendant),
            Some(1),
            "sanity: P0 has 20 life"
        );
        steal(&mut state, ascendant, P1, Timestamp(1_000));
        assert_eq!(
            state.layers().power(ascendant),
            Some(6),
            "the stolen Ascendant reads its new controller's 30 life"
        );
    }

    /// [CR#109.5,613.1b]: a static control-changing ability's "you" is the
    /// current controller of the permanent it is on. P0's Control Magic ("You
    /// control enchanted creature.") is stolen by P1 and then becomes attached
    /// to P1's Grizzly Bears, which gives it a timestamp after the steal
    /// [CR#613.7e]; its layer-2 effect applies after the steal, so P1 keeps
    /// the Bears.
    #[test]
    fn stolen_control_magics_you_is_its_new_controller() {
        let mut state = game();
        let control_magic = permanent(
            &mut state,
            P0,
            "Control Magic",
            vec![Type::Enchantment],
            None,
            vec![Ability::r#static(StaticSpec::Modify(
                Reference::AttachHostOf(Arc::new(Reference::source_parameter())),
                Modification::SetController(Reference::controller_parameter()),
            ))],
        );
        let bears = permanent(
            &mut state,
            P1,
            "Grizzly Bears",
            vec![Type::Creature],
            Some(2),
            vec![],
        );
        steal(&mut state, control_magic, P1, Timestamp(1_000));
        let aura = state.objects.obj_mut(control_magic);
        aura.attached_to = Some(bears);
        aura.timestamp = Timestamp(2_000);
        let view = state.layers();
        assert_eq!(view.controller(control_magic), P1, "sanity: stolen");
        assert_eq!(
            view.controller(bears),
            P1,
            "Control Magic's \"you\" is its controller when its effect applies"
        );
    }
}
