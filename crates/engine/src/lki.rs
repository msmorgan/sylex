//! Last-known information ([CR#603.10a], [CR#608.2h]). A value snapshot of an
//! object captured at a zone change, so triggers and effects can read it after
//! the object itself is gone. The engine never retains dead objects.

use std::collections::HashMap;

use sylex_core::Ident;
use sylex_core::Uint;
use sylex_core::Zone;

use crate::layer::Characteristics;
use crate::layer::LayeredView;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::object::Side;
use crate::player::PlayerId;
use crate::state::GameState;

/// An object as it last existed in the zone it left ([CR#608.2h]), captured
/// the instant before it changed zones. One enumerated record: every fact a
/// snapshot reader may ask of a departed object is a field here, read from the
/// same derivation the live object answered from, so a reader never falls back
/// to the printed card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LkiSnapshot {
    /// The now-stale id — a label/reference token, not a live object.
    pub object: ObjectId,
    /// `CardId` spine: identity and the owner ([CR#108.3]), never
    /// characteristics.
    pub source: ObjectSource,
    /// Its characteristics as the layer system derived them
    /// ([CR#613.1]): types, subtypes, supertypes, colors, name, mana value,
    /// loyalty, power, toughness and abilities, keywords included. `None` for
    /// the one kind of object no card stands behind: an ability on the stack
    /// ([CR#113.1c]) whose source is a player, which is how a created trigger
    /// whose source has left play for good is put on the stack, along with
    /// its copies ([CR#707.10b]). The layer system derives nothing for it.
    pub characteristics: Option<Characteristics>,
    /// Its controller as layer 2 derived it ([CR#613.1b]): a stolen creature
    /// was its thief's. An object without characteristics keeps its base
    /// controller, which no layer changes.
    pub controller: PlayerId,
    /// Which face was up ([CR#712.6c,712.6d]).
    pub side: Side,
    pub tapped: bool,
    pub damage: Uint,
    /// The attachment host it was attached to the instant it left
    /// ([CR#701.3,603.10a]) — mirrors `GameObject::attached_to`. A departed
    /// Aura/Equipment's leaves-the-battlefield ability reads its last host
    /// through this once the live object is gone.
    pub attached_to: Option<ObjectId>,
    /// Counters on the object the instant it left ([CR#122.1,603.10a]) — keyed
    /// by counter name, mirroring `GameObject::counters`. A dies-trigger
    /// reads these once the live object is gone (its id is stale):
    /// Modular's "for each +1/+1 counter on this permanent" count, and
    /// Undying/Persist's "if it had no +1/+1 (resp. -1/-1) counters on it"
    /// intervening-if ([CR#702.93a,702.79a]).
    pub counters: HashMap<Ident, Uint>,
    /// The zone it left.
    pub left: Zone,
}

impl LkiSnapshot {
    /// Snapshot a currently-live object, deriving its characteristics and
    /// controller from the current layer pass.
    ///
    /// # Panics
    ///
    /// Panics on a player proxy (no zone) or a stale `ObjectId`.
    #[must_use]
    pub fn capture(state: &GameState, id: ObjectId) -> Self {
        Self::capture_in(state, &state.layers(), id)
    }

    /// Snapshot a currently-live object against an already-derived `view` of
    /// the same state. Capturing several objects against one view records
    /// them all from one moment, which a simultaneous batch needs
    /// ([CR#704.8]).
    ///
    /// # Panics
    ///
    /// Panics on a player proxy (no zone), a stale `ObjectId`, or a
    /// card-backed object absent from `view` (a view of some other state).
    #[must_use]
    pub fn capture_in(state: &GameState, view: &LayeredView, id: ObjectId) -> Self {
        let o = state.objects.obj(id);
        let (characteristics, controller) = match o.source {
            ObjectSource::Card(_) => (
                Some(
                    view.try_get(id)
                        .unwrap_or_else(|| {
                            panic!("card-backed {id:?} is absent from the layered view")
                        })
                        .clone(),
                ),
                view.controller(id),
            ),
            ObjectSource::Player(_) => (None, o.controller),
        };
        LkiSnapshot {
            object: id,
            source: o.source,
            characteristics,
            controller,
            side: o.side,
            tapped: o.tapped,
            damage: o.total_damage(),
            attached_to: o.attached_to,
            counters: o.counters.clone(),
            left: o.zone.expect("a zoned object has a zone to leave"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use sylex_plugin_v1::plugin::Plugin;

    use super::*;
    use crate::object::ObjectSource;
    use crate::object::Side;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../v1/plugins/canon"),
        )
        .unwrap()
    }

    /// [CR#603.10a]: `LkiSnapshot::capture` records the object's counters at the
    /// instant it leaves — the data Undying/Persist's intervening-if reads off
    /// the dying object. Two +1/+1 counters on a battlefield creature are
    /// captured; the snapshot is decoupled from the live object (clearing the
    /// live map after capture leaves the snapshot untouched).
    #[test]
    fn capture_records_object_counters() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 4],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 4],
                },
            ],
            seed: 5,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 2);

        let snapshot = LkiSnapshot::capture(&state, bear);
        assert_eq!(
            snapshot.counters.get("P1P1Counter").copied(),
            Some(2),
            "the +1/+1 counters are captured on the snapshot"
        );
        assert_eq!(
            snapshot.counters.get("M1M1Counter").copied(),
            None,
            "an absent counter kind is absent on the snapshot"
        );

        // The snapshot is a value copy, independent of the live object.
        state.objects.obj_mut(bear).counters.clear();
        assert_eq!(
            snapshot.counters.get("P1P1Counter").copied(),
            Some(2),
            "the snapshot survives the live counters being cleared"
        );
    }

    /// [CR#608.2h,712.6d]: a transformed Delver of Secrets is captured as the
    /// object it last was — its back face up, named Insectile Aberration, a
    /// 3/2 flier, with the mana value of its front face's {U}.
    #[test]
    fn capture_records_the_face_up_and_its_characteristics() {
        let delver = Arc::new(canon().card("Delver of Secrets").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 5,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card = state.cards.push(delver, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        state.objects.obj_mut(id).side = Side::Back;

        let snapshot = LkiSnapshot::capture(&state, id);
        assert_eq!(snapshot.side, Side::Back);
        let c = snapshot.characteristics.expect("a card-backed object");
        assert_eq!(&*c.name, "Insectile Aberration");
        assert_eq!(c.mana_value, 1);
        assert_eq!(c.power, Some(3));
        assert_eq!(c.toughness, Some(2));
        assert!(
            c.abilities
                .iter()
                .any(|a| crate::layer::ability_is_named(a, &Ident::from("Flying"))),
            "keyword abilities are captured: {:?}",
            c.abilities
        );
    }

    /// [CR#113.1c,707.10b]: a created trigger whose source has left play for
    /// good goes on the stack with its controller as source
    /// (`created_trigger_context`), and so does a copy of it. Targeting it
    /// (Stifle: "Counter target activated or triggered ability.") snapshots
    /// it: no card stands behind it, so it has no derived characteristics,
    /// and its controller is the one it was put on the stack under.
    #[test]
    fn capture_of_a_player_sourced_ability_on_the_stack_has_no_characteristics() {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 5,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        // The same mint `place_one_trigger` makes for such a trigger.
        let ability = state.objects.mint(
            ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Stack),
        );
        let snapshot = LkiSnapshot::capture(&state, ability);
        assert_eq!(snapshot.characteristics, None);
        assert_eq!(snapshot.controller, PlayerId(1));
        assert_eq!(snapshot.left, Zone::Stack);
    }
}
