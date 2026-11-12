//! The layer system ([CR#613]): derived characteristics. Fixtures are real
//! cards from `v1/plugins/canon`, plus `v1/plugins/testing` mocks for the two
//! layer shapes no real card carries (see that plugin's cards/README.md).

use std::path::Path;
use std::sync::Arc;

use sylex_card::Card;
use sylex_core::Color;
use sylex_core::Zone;
use sylex_engine::GameConfig;
use sylex_engine::GameState;
use sylex_engine::ObjectId;
use sylex_engine::PlayerConfig;
use sylex_engine::PlayerId;
use sylex_engine::StartingPlayer;
use sylex_engine::legal_attackers;
use sylex_plugin_v1::plugin::Plugin;

fn plugin(name: &str) -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../v1/plugins/{name}")),
    )
    .unwrap()
}

/// Looks `name` up in canon (real cards) first, then in the testing mocks.
fn card(name: &str) -> Arc<Card> {
    let card = plugin("canon")
        .card(name)
        .or_else(|_| plugin("testing").card(name))
        .unwrap()
        .core;
    Arc::new(card)
}

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
    }
}

fn two_player_with(card_name: &str, seed: u64, deck_size: usize) -> GameState {
    let c = card(card_name);
    let deck = vec![Arc::clone(&c); deck_size];
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: deck.clone() },
            PlayerConfig { deck: deck.clone() },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// True iff `o` is a card-backed object whose face name is `name`.
fn is_named(state: &GameState, o: ObjectId, name: &str) -> bool {
    state.objects.obj(o).card_id().is_some() && face_name(state, o) == name
}

/// P0's deck contains `names`, padded with Grizzly Bears so the opening
/// draw never empties the library; P1 plays a plain Grizzly Bears deck.
fn game_with_p0_cards(names: &[&str], seed: u64) -> GameState {
    let mut p0: Vec<Arc<Card>> = names.iter().map(|n| card(n)).collect();
    while p0.len() < 10 {
        p0.push(card("Grizzly Bears"));
    }
    let p1 = vec![card("Grizzly Bears"); 10];
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// Move the first object named `name` in `player`'s hand or library straight
/// onto the battlefield. Public-fields-only; no engine API widening.
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let p = player.index();
    let obj = if let Some(i) = state.zones.hands[p]
        .iter()
        .position(|&o| is_named(state, o, name))
    {
        state.zones.hands[p].remove(i)
    } else {
        let i = state.zones.libraries[p]
            .iter()
            .position(|&o| is_named(state, o, name))
            .unwrap_or_else(|| panic!("no {name} in P{}'s hand or library", player.0));
        state.zones.libraries[p]
            .remove(i)
            .expect("index in library")
    };
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// [CR#613.1]: with no continuous effects, derived characteristics equal the
/// printed values — the layer system is behavior-preserving at the base.
#[test]
fn base_values_equal_printed() {
    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    let view = state.layers();
    assert_eq!(view.power(bear), Some(2), "printed 2/2 derives power 2");
    assert_eq!(
        view.toughness(bear),
        Some(2),
        "printed 2/2 derives toughness 2"
    );
}

/// [CR#613.4c]: a static "+1/+1 to black creatures" (layer 7c) pumps a black
/// 2/2 to 3/3.
#[test]
fn anthem_pumps_power_and_toughness() {
    let mut state = game_with_p0_cards(&["Bad Moon", "Walking Corpse"], 1);
    let zombie = force_onto_battlefield(&mut state, PlayerId(0), "Walking Corpse");
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon");
    let view = state.layers();
    assert_eq!(view.power(zombie), Some(3), "Bad Moon +1/+1 → 3 power");
    assert_eq!(
        view.toughness(zombie),
        Some(3),
        "Bad Moon +1/+1 → 3 toughness"
    );
}

/// [CR#613.4]: a 7b base set applies before all 7c modification, so Humility's
/// "base 1/1" + Bad Moon's "+1/+1" = 2/2 regardless of timestamps.
#[test]
fn base_set_applies_before_modify() {
    let mut state = game_with_p0_cards(&["Humility", "Bad Moon", "Walking Corpse"], 1);
    let zombie = force_onto_battlefield(&mut state, PlayerId(0), "Walking Corpse");
    let _humility = force_onto_battlefield(&mut state, PlayerId(0), "Humility");
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon");
    let view = state.layers();
    assert_eq!(
        view.power(zombie),
        Some(2),
        "base 1/1 (7b), then +1/+1 (7c)"
    );
    assert_eq!(view.toughness(zombie), Some(2));
}

/// [CR#611.2c],[CR#514.2]: a one-shot "+3/+3 until end of turn" pumps a
/// creature, then wears off at Cleanup.
#[test]
fn one_shot_pump_expires_at_cleanup() {
    use sylex_core::Count;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![
            Modification::Power(sylex_core::NumericOp::Up(Count::Literal(3))),
            Modification::Toughness(sylex_core::NumericOp::Up(Count::Literal(3))),
        ],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    assert_eq!(state.layers().power(bear), Some(5), "2/2 +3/+3 → 5");

    state.expire_end_of_turn();
    assert_eq!(
        state.layers().power(bear),
        Some(2),
        "pump gone after cleanup"
    );
}

/// [CR#613.4c]: a static "-1/-1" (layer 7c) lowers a 2/2 to 1/1.
#[test]
fn negative_modify_lowers_power_and_toughness() {
    use sylex_core::Count;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![
            Modification::Power(sylex_core::NumericOp::Down(Count::Literal(1))),
            Modification::Toughness(sylex_core::NumericOp::Down(Count::Literal(1))),
        ],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    let view = state.layers();
    assert_eq!(view.power(bear), Some(1), "-1/-1 → 1 power");
    assert_eq!(view.toughness(bear), Some(1), "-1/-1 → 1 toughness");
}

/// [CR#613.1f]: a static "creatures gain trample" (layer 6) grants the keyword.
/// The granter is a mock — symmetric "all creatures have <native keyword>"
/// statics don't exist in real Magic.
#[test]
fn static_grants_keyword() {
    use sylex_core::KeywordAbility;
    use sylex_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample granter"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _granter = force_onto_battlefield(&mut state, PlayerId(0), "Trample granter");
    assert!(
        has_keyword(&state.layers(), bear, &KeywordAbility::Trample),
        "the bear gains trample from the static ([CR#613.1f])"
    );
}

/// [CR#611.3a,613.1f]: a conditional static is re-evaluated continuously
/// against its own carrier. The testing-plugin fixture grants itself trample
/// while untapped; changing the stored tap status makes the grant disappear on
/// the next derived view.
#[test]
fn conditional_static_tracks_source_state() {
    use sylex_core::KeywordAbility;
    use sylex_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Conditional trample source"], 1);
    let source = force_onto_battlefield(&mut state, PlayerId(0), "Conditional trample source");

    assert!(
        has_keyword(&state.layers(), source, &KeywordAbility::Trample),
        "the untapped source satisfies its condition and gains trample"
    );

    state.objects.obj_mut(source).tapped = true;
    assert!(
        !has_keyword(&state.layers(), source, &KeywordAbility::Trample),
        "tapping the source makes the conditional grant disappear"
    );
}

/// [CR#611.3a,613.8a]: a self-referential layer-6 grant — "creatures with
/// trample have trample" — applies exactly once. The affected set is whatever
/// has trample when the effect applies, so the grant can't feed itself: no
/// fixpoint loop, trample-less creatures stay out of the set, and a printed
/// trampler ends up with a redundant ([CR#702.19g]) second instance.
#[test]
fn self_referential_grant_applies_once() {
    use sylex_core::Ability;
    use sylex_core::KeywordAbility;
    use sylex_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample tautology", "Fangren Hunter"], 1);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _tautology = force_onto_battlefield(&mut state, PlayerId(0), "Trample tautology");

    let view = state.layers();
    let instances = view
        .get(trampler)
        .abilities
        .iter()
        .filter(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample)))
        .count();
    assert_eq!(
        instances, 2,
        "printed trample + the granted (redundant, [CR#702.19g]) one"
    );
    assert!(
        !has_keyword(&view, bear, &KeywordAbility::Trample),
        "a creature without trample is never in the affected set ([CR#611.3a])"
    );
}

/// [CR#613.1f]: Humility's "creatures lose all abilities" blanks a printed
/// keyword.
#[test]
fn lose_all_abilities_blanks_keyword() {
    use sylex_core::KeywordAbility;
    use sylex_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Fangren Hunter", "Humility"], 1);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let _humility = force_onto_battlefield(&mut state, PlayerId(0), "Humility");
    assert!(
        !has_keyword(&state.layers(), trampler, &KeywordAbility::Trample),
        "lose-all-abilities removes the printed trample ([CR#613.1f])"
    );
}

/// [CR#613.6]: an effect that REPLACES an enchantment's type with Creature (L4)
/// AND sets its P/T (L7b) applies the P/T to the now-creature — the target set
/// locks at L4 and the L7b part rides along, even though the object no longer
/// matches the effect's `Matching(Enchantment)` filter by L7b. The animator is
/// a mock — no real static type-replaces with a literal P/T set.
#[test]
fn type_change_then_set_pt_locks_in() {
    use sylex_core::Type;

    let mut state = game_with_p0_cards(&["Animate enchantments", "Moonlit Wake"], 1);
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");

    let view = state.layers();
    assert!(
        view.get(ench).has_type(Type::Creature),
        "the enchantment became a creature (layer 4)"
    );
    assert_eq!(
        view.power(ench),
        Some(4),
        "and its P/T was set to 4/4 (layer 7b, riding the layer-4-locked set, [CR#613.6])"
    );
}

/// [CR#613.1e] layer 5: Darkest Hour's "all creatures are black" replaces a
/// creature's derived colors.
#[test]
fn static_sets_color() {
    let mut state = game_with_p0_cards(&["Darkest Hour"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _hour = force_onto_battlefield(&mut state, PlayerId(0), "Darkest Hour");

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Black),
        "the creature is now black (layer 5)"
    );
    assert!(
        !view.get(bear).colors.contains(&Color::Green),
        "\"are black\" replaces the printed green (layer 5)"
    );
}

/// [CR#613.6] for layer 5: Bad Moon ("+1/+1 to black creatures") catches a
/// creature that is only black because Darkest Hour painted it black THIS pass
/// — `matches_derived` evaluates the scope against the derived color, not the
/// printed face.
#[test]
fn anthem_catches_creature_painted_black() {
    let mut state = game_with_p0_cards(&["Darkest Hour", "Bad Moon"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears"); // base green
    let _hour = force_onto_battlefield(&mut state, PlayerId(0), "Darkest Hour"); // creatures are black (L5)
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon"); // black creatures +1/+1 (L7c)

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Black),
        "painted black (L5)"
    );
    assert_eq!(
        view.power(bear),
        Some(3),
        "Bad Moon caught the now-black creature via derived-color matching ([CR#613.6])"
    );
}

/// [CR#613.4c],[CR#122]: +1/+1 counters modify P/T in layer 7c.
#[test]
fn minus_one_counters_shrink() {
    use sylex_core::Ident;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    state.counter_decls = plugin("builtin").counters;
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("M1M1Counter"), 1);

    let view = state.layers();
    assert_eq!(view.power(bear), Some(1), "2/2 + one M1M1Counter → 1 power");
    assert_eq!(
        view.toughness(bear),
        Some(1),
        "2/2 + one M1M1Counter → 1 toughness"
    );
}

/// A +1/+1 counter on a permanent with no P/T (a non-creature) does nothing —
/// the conferred boost only applies where P/T is `Some`.
#[test]
fn counter_on_non_creature_does_nothing() {
    use sylex_core::Ident;

    let mut state = game_with_p0_cards(&["Moonlit Wake"], 1);
    state.counter_decls = plugin("builtin").counters;
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");
    state
        .objects
        .obj_mut(ench)
        .counters
        .insert(Ident::from("P1P1Counter"), 3);

    let view = state.layers();
    assert_eq!(
        view.power(ench),
        None,
        "a counter on a non-creature confers no P/T"
    );
    assert_eq!(view.toughness(ench), None);
}

/// [CR#613.4c]: +1/+1 and -1/-1 counters combine as a net delta — two
/// conferred `Continuous` boosts (+2/+2 and -1/-1) sum in layer 7c.
#[test]
fn mixed_counters_net_delta() {
    use sylex_core::Ident;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    state.counter_decls = plugin("builtin").counters;
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("P1P1Counter"), 2);
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("M1M1Counter"), 1);

    let view = state.layers();
    assert_eq!(
        view.power(bear),
        Some(3),
        "2/2 + two P1P1Counter - one M1M1Counter → net +1 → 3"
    );
    assert_eq!(view.toughness(bear), Some(3));
}

/// [CR#122.1a]: a `P1P1Counter` pumps P/T via its conferred `Continuous` boost,
/// gathered from the counter registry (`state.counter_decls`) — the
/// data-driven path, not a hardcoded `"+1/+1"` read. The boost scales by the
/// live count via the baked `CounterCount`.
#[test]
fn data_driven_plus_one_counter_pumps() {
    use sylex_core::Ident;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    state.counter_decls = plugin("builtin").counters;
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("P1P1Counter"), 2);

    let view = state.layers();
    assert_eq!(view.power(bear), Some(4), "2/2 + two P1P1Counter → 4 power");
    assert_eq!(view.toughness(bear), Some(4));
}

/// [CR#122.1b,613.1f]: a KEYWORD counter confers its keyword via the SAME
/// data-driven mechanism as a +1/+1 counter — a `Continuous` boost, only the
/// `Modification` differs (`GainAbility(Keyword(Flying))` vs `Power(Up)`). A
/// `FlyingCounter` makes its bearer fly (layer 6).
#[test]
fn keyword_counter_confers_its_keyword() {
    use sylex_core::Ident;
    use sylex_engine::has_keyword_named;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    state.counter_decls = plugin("builtin").counters;
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    assert!(
        !has_keyword_named(&state.layers(), bear, "Flying"),
        "a plain Grizzly Bears does not fly"
    );

    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("FlyingCounter"), 1);
    assert!(
        has_keyword_named(&state.layers(), bear, "Flying"),
        "a FlyingCounter grants flying"
    );
}

/// [CR#305.6,611.3]: a subtype ADDED by a layer-4 effect brings its
/// registry-conferred abilities along — the derived ability list gains the
/// keyword the added subtype confers, not just the printed subtypes' confers.
/// The dual of the fold handling the PRINTED subtypes' confers, now for a
/// subtype granted at layer 4. Uses a mock registry subtype conferring Trample
/// so it stands alone from any card's printed text (and from the T4
/// Creature-type combat capability).
#[test]
fn layer_added_subtype_confers_its_keyword() {
    use sylex_core::Ability;
    use sylex_core::CollectionOp;
    use sylex_core::Duration;
    use sylex_core::KeywordAbility;
    use sylex_core::Modification;
    use sylex_core::Property;
    use sylex_core::Subtype;
    use sylex_core::Type;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;
    use sylex_engine::has_keyword;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    // Register a Creature subtype whose membership confers Trample
    // ([CR#305.6]).
    state.subtypes.insert(
        "Trampler".into(),
        Subtype {
            name: "Trampler".into(),
            types: vec![Type::Creature].into(),
            confers: vec![Property::Ability(Arc::new(Ability::Keyword(
                KeywordAbility::Trample,
            )))]
            .into(),
        },
    );
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    // A vanilla bear does not trample.
    assert!(
        !has_keyword(&state.layers(), bear, &KeywordAbility::Trample),
        "sanity: a plain Grizzly Bears has no trample"
    );

    // Grant the registered subtype via a layer-4 `Subtypes(Add)` effect.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::Subtypes(CollectionOp::Add("Trampler".into()))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    assert!(
        has_keyword(&state.layers(), bear, &KeywordAbility::Trample),
        "the layer-4-added Trampler subtype's conferred Trample joins the derived \
         abilities ([CR#305.6,611.3])"
    );
}

/// [CR#508.1a]: a permanent animated into a creature by a layer-4 effect is a
/// legal attacker — combat legality reads the derived view, not the printed
/// type.
#[test]
fn animated_enchantment_can_attack() {
    let mut state = game_with_p0_cards(&["Animate enchantments", "Moonlit Wake"], 1);
    // The layer-4 `CardTypes(Set(["Creature"]))` carries only the bare
    // `Creature` name; the engine recovers the type's combat-capability
    // `confers` (its `May(Attack)` grant) from the type registry, so wire
    // it in ([CR#300.1]). Without it the animator adds a name-only Creature
    // type with no confers and the fold has nothing to contribute.
    state.types = plugin("canon").types;
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");
    // force_onto_battlefield leaves it untapped and not summoning-sick (mint
    // defaults).

    assert!(
        state
            .layers()
            .get(ench)
            .has_type(sylex_core::Type::Creature),
        "sanity: the enchantment derives as a creature"
    );
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&ench),
        "an animated-into-creature permanent can attack ([CR#508.1a] over the derived type)"
    );
}

/// How many Combatant role conferrals the object's DERIVED card types carry
/// ([CR#113.12]). The role is a quality of the object, not an ability, so it
/// lives in the type's ability-free `Property::Static` confers rather than in
/// the ability list; the permanent gate it wears is looked through here.
fn conferred_combatant_rows(state: &sylex_engine::GameState, id: ObjectId) -> usize {
    use sylex_core::Property;
    use sylex_core::Role;
    use sylex_core::StaticSpec;

    fn is_role(spec: &StaticSpec) -> bool {
        match spec {
            StaticSpec::Conditionally(_, inner) => is_role(inner),
            StaticSpec::Role { role, .. } => *role == Role::Combatant,
            _ => false,
        }
    }

    state
        .layers()
        .get(id)
        .card_types
        .iter()
        .flat_map(|t| t.confers.iter())
        .filter(|p| matches!(p, Property::Static(s) if is_role(&s.body)))
        .count()
}

/// [CR#305.6,611.3]: a permanent that LOSES its `Creature` type via a layer-4
/// effect stops playing the Combatant role — the mirror image of
/// `animated_enchantment_can_attack`. Because conferral is recomputed from
/// the object's CURRENT `card_types` every pass (not cached once at push and
/// frozen), the role disappears the moment the type does, and with it the
/// attack permission the role derives.
#[test]
fn losing_creature_type_removes_the_combatant_role() {
    use sylex_core::CollectionOp;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::Type;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    // Sanity: before the effect, the bear is a creature and a legal attacker.
    assert!(
        state.layers().get(bear).has_type(Type::Creature),
        "sanity: a plain Grizzly Bears is a creature"
    );
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&bear),
        "sanity: a plain Grizzly Bears can attack"
    );

    // Strip the Creature type via a layer-4 `CardTypes(Set(..))` effect. We
    // don't need the type registry populated: we assert the ABSENCE of the
    // old confer, and the new "Enchantment" type's own confers (if any) are
    // irrelevant here.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::CardTypes(CollectionOp::Set(
            vec!["Enchantment".into()].into(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    let view = state.layers();
    assert!(
        !view.get(bear).has_type(Type::Creature),
        "the bear is no longer a creature once its type is overwritten"
    );
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a permanent turned into a non-creature is not a legal attacker \
         ([CR#508.1a])"
    );
    drop(view);
    assert_eq!(
        conferred_combatant_rows(&state, bear),
        0,
        "the Creature type's Combatant conferral is gone, not stuck from a \
         frozen cache — conferral is recomputed from CURRENT card_types every \
         pass, not cached once at push ([CR#305.6,611.3])"
    );
}

/// [CR#305.6,611.3]: guards against doubling — a PRINTED creature's
/// `Creature`-type-conferred Combatant role appears exactly once, not
/// twice. The base (`printed_of_face`) carries NO type/subtype confer, so the
/// layer-4 fold (`fold_conferred_abilities`) is the ONLY path that pushes it,
/// and it pushes exactly once per pass (the fixpoint reseeds base fresh each
/// pass rather than accumulating onto a prior pass's result). If the fold ever
/// ran twice per pass, or the base regained a cached copy, this count would go
/// 1 → 2.
#[test]
fn printed_creature_grant_is_not_doubled_by_the_fold() {
    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    assert_eq!(
        conferred_combatant_rows(&state, bear),
        1,
        "the Creature type's Combatant conferral appears exactly once: the \
         derived card-type list carries `Creature` once per fixpoint pass, so \
         the role is read once ([CR#305.6,611.3])"
    );
}

/// [CR#613.1b]: a layer-2 control-change effect ("you gain control") makes the
/// derived controller the effect's controller. The object's base controller is
/// unchanged — control change is a continuous effect, not a mutation.
#[test]
fn gain_control_changes_derived_controller() {
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::Reference;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(0),
        "base controller is P0"
    );

    // P1 "gains control of" the bear (Threaten-style). Per [CR#611.2c] the
    // effect's `You` is locked to its controller (P1) at creation.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::SetController(Reference::Reg(
            sylex_core::RefId(1),
        ))],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(1),
        "layer 2 derives the new controller ([CR#613.1b])"
    );
}

/// [CR#613.1b],[CR#613.5]: control change is continuous — when the effect
/// expires the derived controller reverts to the base controller.
#[test]
fn gained_control_reverts_when_effect_expires() {
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::Reference;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::SetController(Reference::Reg(
            sylex_core::RefId(1),
        ))],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    assert_eq!(state.layers().controller(bear), PlayerId(1), "stolen");

    state.expire_end_of_turn();
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(0),
        "control reverts after the effect expires ([CR#613.5])"
    );
}

/// [CR#613.8a,613.8b]: within layer 4, an effect whose affected set DEPENDS on
/// another applies after it, overriding timestamp order. `Instruction` B
/// ("enchantments are also artifacts", EARLIER timestamp) depends on effect A
/// ("creatures are also enchantments", LATER timestamp): applying A adds the
/// bear to B's affected set. So A applies first despite its later timestamp,
/// and the bear ends up an artifact — which a naive timestamp order (B before
/// A) would miss.
#[test]
fn dependency_orders_dependent_effect_after_its_dependency() {
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::Predicate;
    use sylex_core::Type;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    // A (LATER timestamp): creatures are also enchantments.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(2_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Floating(Arc::new(sylex_core::Region::candidate(Predicate::r#type(
            Type::Creature,
        )))),
        changes: vec![Modification::CardTypes(sylex_core::CollectionOp::Add(
            Type::Enchantment.name(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    // B (EARLIER timestamp): enchantments are also artifacts. Depends on A.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Floating(Arc::new(sylex_core::Region::candidate(Predicate::r#type(
            Type::Enchantment,
        )))),
        changes: vec![Modification::CardTypes(sylex_core::CollectionOp::Add(
            Type::Artifact.name(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    let view = state.layers();
    assert!(
        view.get(bear).has_type(Type::Enchantment),
        "A makes the bear an enchantment"
    );
    assert!(
        view.get(bear).has_type(Type::Artifact),
        "B depends on A and applies after it, so the bear becomes an artifact ([CR#613.8b])"
    );
}

/// [CR#613.7],[CR#613.8]: independent same-layer effects keep timestamp order —
/// the dependency pass must not reorder effects that don't change each other's
/// affected set. Two color sets on the same creature: the later timestamp wins.
#[test]
fn independent_effects_keep_timestamp_order() {
    use sylex_core::Color;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    // Earlier: becomes red. Later: becomes blue. Both layer 5, independent.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::Colors(sylex_core::CollectionOp::Set(
            vec![Color::Red].into(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(2_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::Colors(sylex_core::CollectionOp::Set(
            vec![Color::Blue].into(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    let view = state.layers();
    assert_eq!(
        view.get(bear).colors.as_slice(),
        [Color::Blue],
        "later timestamp wins for independent effects ([CR#613.7])"
    );
}

/// [CR#613.8b]: a dependency loop is ignored and its members apply in timestamp
/// order (and the pass must terminate). "Creatures are also enchantments" (A,
/// earlier) and "enchantments are also creatures" (B, later) each change the
/// other's affected set — a 2-cycle. Falling back to timestamp order, A applies
/// first: it catches the printed creature, B then catches the printed
/// enchantment (and the now-enchantment creature).
#[test]
fn dependency_loop_falls_back_to_timestamp() {
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::Predicate;
    use sylex_core::Type;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;
    use sylex_engine::Timestamp;

    let mut state = game_with_p0_cards(&["Bad Moon"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon"); // an enchantment

    // A (earlier): creatures are also enchantments.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Floating(Arc::new(sylex_core::Region::candidate(Predicate::r#type(
            Type::Creature,
        )))),
        changes: vec![Modification::CardTypes(sylex_core::CollectionOp::Add(
            Type::Enchantment.name(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    // B (later): enchantments are also creatures. Mutually dependent with A.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(2_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Floating(Arc::new(sylex_core::Region::candidate(Predicate::r#type(
            Type::Enchantment,
        )))),
        changes: vec![Modification::CardTypes(sylex_core::CollectionOp::Add(
            Type::Creature.name(),
        ))],
        duration: Duration::EndOfGame,
        rows: vec![],
        origin: None,
        is_cda: false,
    });

    // Must terminate (no hang) and resolve in timestamp order.
    let view = state.layers();
    assert!(
        view.get(bear).has_type(Type::Enchantment),
        "A (earlier) catches the creature: the bear becomes an enchantment"
    );
    assert!(
        view.get(moon).has_type(Type::Creature),
        "B then catches the enchantment: Bad Moon becomes a creature ([CR#613.8b] loop → timestamp)"
    );
}

// ---------------------------------------------------------------------------
// Humility, Opalescence and effect continuation ([CR#613.6])
// ---------------------------------------------------------------------------

/// A core fixture built from a real card's Oracle text, for the cards the v1
/// corpus cannot load.
fn fixture(
    name: &str,
    mana_cost: Vec<sylex_core::ManaSymbol>,
    types: &[sylex_core::Type],
    subtypes: &[&str],
    power_toughness: Option<(i32, i32)>,
    abilities: Vec<sylex_core::Ability>,
) -> Arc<Card> {
    use sylex_core::StatValue;
    Arc::new(Card::Normal(sylex_card::CardFace::from(
        sylex_card::Characteristics {
            name: name.into(),
            mana_cost: Arc::<[sylex_core::ManaSymbol]>::from(mana_cost).into(),
            types: types.iter().map(|t| t.def()).collect(),
            subtypes: subtypes
                .iter()
                .map(|s| sylex_core::Subtype {
                    name: (*s).into(),
                    types: Vec::new().into(),
                    confers: Vec::new().into(),
                })
                .collect(),
            power: power_toughness.map(|(p, _)| StatValue::Number(p)),
            toughness: power_toughness.map(|(_, t)| StatValue::Number(t)),
            abilities,
            ..sylex_card::Characteristics::default()
        },
    )))
}

/// `{generic}` followed by `white` white pips.
fn white_cost(generic: u32, white: usize) -> Vec<sylex_core::ManaSymbol> {
    let mut cost = vec![sylex_core::ManaSymbol::Simple(
        sylex_core::SimpleManaSymbol::Generic(generic),
    )];
    cost.extend(std::iter::repeat_n(
        sylex_core::ManaSymbol::from(Color::White),
        white,
    ));
    cost
}

/// The candidate filter region of a static's "each …" selection: register 0
/// is the candidate, 1 the static's source (`~`), 2 its controller (`you`).
fn static_candidate_filter(
    body: sylex_core::Predicate,
) -> Arc<sylex_core::Region<sylex_core::Predicate>> {
    use sylex_core::DefId;
    use sylex_core::Kind;
    use sylex_core::Param;
    use sylex_core::Provenance;
    Arc::new(sylex_core::Region::new(
        Arc::from([
            Param {
                def: DefId(0),
                kind: Kind::Entity,
                provenance: Provenance::Candidate(sylex_core::Domain::Entity),
            },
            Param {
                def: DefId(1),
                kind: Kind::Entity,
                provenance: Provenance::Source,
            },
            Param {
                def: DefId(2),
                kind: Kind::Entity,
                provenance: Provenance::Controller,
            },
        ]),
        body,
    ))
}

/// A static "each object matching `filter` gets `changes`".
fn each_static(
    filter: sylex_core::Predicate,
    changes: Vec<sylex_core::Modification>,
) -> sylex_core::Ability {
    use sylex_core::Modification;
    use sylex_core::Reference;
    use sylex_core::StaticSpec;
    sylex_core::Ability::r#static(StaticSpec::Each(
        sylex_core::Selection::SelectAll(static_candidate_filter(filter)),
        Arc::new(sylex_core::Region::candidate(StaticSpec::Modify(
            Reference::Reg(sylex_core::RefId(0)),
            Modification::Several(changes.into()),
        ))),
    ))
}

/// Opalescence ({2}{W}{W} Enchantment): "Each other non-Aura enchantment is a
/// creature in addition to its other types and has base power and base
/// toughness each equal to its mana value." "Its mana value" names the
/// `Each` body's candidate, register 0: the enchantment being modified.
fn opalescence() -> Arc<Card> {
    use sylex_core::CharacteristicPredicate;
    use sylex_core::CollectionOp;
    use sylex_core::Count;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Predicate;
    use sylex_core::Reference;
    use sylex_core::Stat;
    use sylex_core::StatValue;
    use sylex_core::Type;
    let its_mana_value = || {
        NumericOp::Set(StatValue::Count(Count::StatOf(
            Reference::Reg(sylex_core::RefId(0)),
            Stat::ManaValue,
        )))
    };
    let animate = each_static(
        Predicate::And(
            vec![
                Predicate::r#type(Type::Enchantment),
                Predicate::Not(Arc::new(Predicate::Characteristic(
                    CharacteristicPredicate::Subtype(sylex_core::SubtypeRef::named("Aura".into())),
                ))),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(sylex_core::RefId(
                    1,
                ))))),
            ]
            .into(),
        ),
        vec![
            Modification::CardTypes(CollectionOp::Add(Type::Creature.name())),
            Modification::Power(its_mana_value()),
            Modification::Toughness(its_mana_value()),
        ],
    );
    fixture(
        "Opalescence",
        white_cost(2, 2),
        &[Type::Enchantment],
        &[],
        None,
        vec![animate],
    )
}

/// Worship ({3}{W} Enchantment). Its one ability, a damage replacement, is
/// left off: these tests read only its types and power and toughness.
fn worship() -> Arc<Card> {
    fixture(
        "Worship",
        white_cost(3, 1),
        &[sylex_core::Type::Enchantment],
        &[],
        None,
        vec![],
    )
}

/// Story Circle ({1}{W}{W} Enchantment). Its abilities are left off: these
/// tests read only its types and power and toughness.
fn story_circle() -> Arc<Card> {
    fixture(
        "Story Circle",
        white_cost(1, 2),
        &[sylex_core::Type::Enchantment],
        &[],
        None,
        vec![],
    )
}

/// Sphere of Safety ({4}{W} Enchantment). Its one ability, an attack tax, is
/// left off: these tests read only its types and power and toughness.
fn sphere_of_safety() -> Arc<Card> {
    fixture(
        "Sphere of Safety",
        white_cost(4, 1),
        &[sylex_core::Type::Enchantment],
        &[],
        None,
        vec![],
    )
}

/// Battle Mastery ({2}{W} Enchantment — Aura): "Enchanted creature has double
/// strike." Its enchant keyword is left off; the test attaches it by hand.
fn battle_mastery() -> Arc<Card> {
    use sylex_core::Ability;
    use sylex_core::KeywordAbility;
    use sylex_core::Modification;
    use sylex_core::Reference;
    use sylex_core::StaticSpec;
    fixture(
        "Battle Mastery",
        white_cost(2, 1),
        &[sylex_core::Type::Enchantment],
        &["Aura"],
        None,
        vec![Ability::r#static(StaticSpec::Modify(
            Reference::AttachHostOf(Arc::new(Reference::Reg(sylex_core::RefId(0)))),
            Modification::GainAbility(Arc::new(Ability::Keyword(KeywordAbility::DoubleStrike))),
        ))],
    )
}

/// Dryad of the Ilysian Grove ({2}{G} Enchantment Creature — Nymph Dryad,
/// 2/4): "Lands you control are every basic land type in addition to their
/// other types." Its additional-land ability is left off; the test reads only
/// the land-type effect.
fn dryad_of_the_ilysian_grove() -> Arc<Card> {
    use sylex_core::CollectionOp;
    use sylex_core::Modification;
    use sylex_core::Predicate;
    use sylex_core::Reference;
    use sylex_core::RelationPredicate;
    use sylex_core::Type;
    let lands_you_control = Predicate::And(
        vec![
            Predicate::r#type(Type::Land),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::Reg(sylex_core::RefId(2)),
            )))),
        ]
        .into(),
    );
    let every_basic_land_type = BASIC_LAND_TYPES
        .iter()
        .map(|name| {
            Modification::Subtypes(CollectionOp::Add(sylex_core::SubtypeRef::named(
                (*name).into(),
            )))
        })
        .collect();
    fixture(
        "Dryad of the Ilysian Grove",
        vec![
            sylex_core::ManaSymbol::Simple(sylex_core::SimpleManaSymbol::Generic(2)),
            sylex_core::ManaSymbol::from(Color::Green),
        ],
        &[Type::Enchantment, Type::Creature],
        &["Nymph", "Dryad"],
        Some((2, 4)),
        vec![each_static(lands_you_control, every_basic_land_type)],
    )
}

const BASIC_LAND_TYPES: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];

/// A game whose P0 deck holds `cards` (padded with Grizzly Bears), and whose
/// P1 plays Grizzly Bears.
fn game_with_p0_deck(mut p0: Vec<Arc<Card>>) -> GameState {
    while p0.len() < 10 {
        p0.push(card("Grizzly Bears"));
    }
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![card("Grizzly Bears"); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// Put P0's cards named `order` onto the battlefield, giving each a later
/// timestamp than the one before it [CR#613.7d]. Returns their ids in order.
fn enter_in_order(state: &mut GameState, order: &[&str]) -> Vec<ObjectId> {
    order
        .iter()
        .zip(10_000..)
        .map(|(name, stamp)| {
            let id = force_onto_battlefield(state, PlayerId(0), name);
            state.objects.obj_mut(id).timestamp = sylex_engine::Timestamp(stamp);
            id
        })
        .collect()
}

/// Assert `id` is an enchantment creature with base `pt`/`pt` and no
/// abilities.
fn assert_blank_animated(view: &sylex_engine::LayeredView, id: ObjectId, pt: i32, what: &str) {
    use sylex_core::Type;
    let c = view.get(id);
    assert!(
        c.has_type(Type::Creature) && c.has_type(Type::Enchantment),
        "{what} is a creature that is still an enchantment"
    );
    assert!(c.abilities.is_empty(), "{what} has no abilities");
    assert_eq!(
        (view.power(id), view.toughness(id)),
        (Some(pt), Some(pt)),
        "{what} is {pt}/{pt}"
    );
}

/// Opalescence sets each animated enchantment's base power and toughness to
/// that enchantment's own mana value [CR#613.4b,202.3]: Story Circle (mana
/// value 3) is 3/3 and Sphere of Safety (mana value 5) is 5/5, never
/// Opalescence's own 4.
#[test]
fn opalescence_reads_each_enchantments_own_mana_value() {
    let mut state = game_with_p0_deck(vec![opalescence(), story_circle(), sphere_of_safety()]);
    let ids = enter_in_order(
        &mut state,
        &["Opalescence", "Story Circle", "Sphere of Safety"],
    );
    let [_opal, circle, sphere] = ids[..] else { unreachable!() };

    let view = state.layers();
    assert_blank_animated(&view, circle, 3, "Story Circle");
    assert_blank_animated(&view, sphere, 5, "Sphere of Safety");
}

/// Humility's 2009-10-01 ruling, Opalescence entered first: "if Opalescence,
/// Humility, and Worship are on the battlefield and Opalescence entered before
/// Humility, the following is true: Layer 4: Humility and Worship each become
/// creatures that are still enchantments. (Opalescence). Layer 6: Humility and
/// Worship each lose their abilities. (Humility) Layer 7b: Humility becomes
/// 4/4 and Worship becomes 4/4. (Opalescence). Humility becomes 1/1 and
/// Worship becomes 1/1 (Humility)."
///
/// Re-derived from the current CR: Opalescence animates Humility and Worship
/// in layer 4 [CR#613.1d]; Humility strips both, itself included, in layer 6
/// [CR#613.1f], yet its effect keeps applying in layer 7b [CR#613.6]; within
/// 7b the earlier Opalescence sets 4/4 and the later Humility then sets 1/1
/// [CR#613.4b,613.7]. Opalescence is not a creature and keeps its ability.
#[test]
fn humility_after_opalescence_leaves_animated_enchantments_1_1() {
    let mut state = game_with_p0_deck(vec![opalescence(), card("Humility"), worship()]);
    let ids = enter_in_order(&mut state, &["Opalescence", "Humility", "Worship"]);
    let [opal, humility, worship] = ids[..] else { unreachable!() };

    let view = state.layers();
    assert_blank_animated(&view, humility, 1, "Humility");
    assert_blank_animated(&view, worship, 1, "Worship");
    assert!(
        !view.get(opal).has_type(sylex_core::Type::Creature),
        "Opalescence does not animate itself"
    );
    assert_eq!(
        view.get(opal).abilities.len(),
        1,
        "Opalescence keeps its ability"
    );
}

/// Humility's 2009-10-01 ruling, Humility entered first: "if Humility entered
/// before Opalescence, the following is true: Layer 4: Humility and Worship
/// each become creatures that are still enchantments (Opalescence). Layer 6:
/// Humility and Worship each lose their abilities (Humility). Layer 7b:
/// Humility becomes 1/1 and Worship becomes 1/1 (Humility). Humility becomes
/// 4/4 and Worship becomes 4/4 (Opalescence)."
///
/// Re-derived from the current CR: layers 4 and 6 as in the other order; in 7b
/// the earlier Humility sets 1/1 and the later Opalescence then sets 4/4
/// [CR#613.4b,613.6,613.7].
#[test]
fn humility_before_opalescence_leaves_animated_enchantments_4_4() {
    let mut state = game_with_p0_deck(vec![opalescence(), card("Humility"), worship()]);
    let ids = enter_in_order(&mut state, &["Humility", "Opalescence", "Worship"]);
    let [humility, _opal, worship] = ids[..] else { unreachable!() };

    let view = state.layers();
    assert_blank_animated(&view, humility, 4, "Humility");
    assert_blank_animated(&view, worship, 4, "Worship");
}

/// Humility and two Opalescences entering in `order`; returns the view and the
/// ids of Humility, the first Opalescence and the second Opalescence.
fn humility_and_two_opalescences(
    order: &[&str; 3],
) -> (sylex_engine::LayeredView, ObjectId, ObjectId, ObjectId) {
    let mut state = game_with_p0_deck(vec![opalescence(), opalescence(), card("Humility")]);
    let ids = enter_in_order(&mut state, order);
    let humility = ids[order.iter().position(|n| *n == "Humility").unwrap()];
    let mut opals = ids.iter().zip(order).filter(|(_, n)| **n == "Opalescence");
    let first = *opals.next().unwrap().0;
    let second = *opals.next().unwrap().0;
    (state.layers(), humility, first, second)
}

/// Humility's and Opalescence's 2006-02-01 ruling: "With a Humility and two
/// Opalescences on the battlefield, if Humility has the latest timestamp, then
/// all creatures are 1/1 with no abilities."
///
/// Re-derived from the current CR: each Opalescence animates Humility and the
/// other Opalescence in layer 4 [CR#613.1d], so all three are creatures that
/// Humility strips in layer 6 [CR#613.1f]; each effect, started before the
/// stripping, continues into 7b [CR#613.6], where Humility, latest, sets 1/1
/// last [CR#613.7].
#[test]
fn humility_latest_among_two_opalescences_makes_all_1_1() {
    let (view, humility, first, second) =
        humility_and_two_opalescences(&["Opalescence", "Opalescence", "Humility"]);
    assert_blank_animated(&view, humility, 1, "Humility");
    assert_blank_animated(&view, first, 1, "the first Opalescence");
    assert_blank_animated(&view, second, 1, "the second Opalescence");
}

/// The 2006-02-01 ruling: "If the timestamp order is Opalescence, Humility,
/// Opalescence, the second Opalescence is 1/1, and the Humility and first
/// Opalescence are 4/4."
///
/// Re-derived from the current CR: layers 4 and 6 as above; in 7b the first
/// Opalescence sets Humility and the second Opalescence 4/4, Humility sets all
/// three 1/1, and the second Opalescence sets Humility and the first
/// Opalescence 4/4 [CR#613.6,613.7].
#[test]
fn humility_between_two_opalescences_leaves_the_second_1_1() {
    let (view, humility, first, second) =
        humility_and_two_opalescences(&["Opalescence", "Humility", "Opalescence"]);
    assert_blank_animated(&view, humility, 4, "Humility");
    assert_blank_animated(&view, first, 4, "the first Opalescence");
    assert_blank_animated(&view, second, 1, "the second Opalescence");
}

/// The 2006-02-01 ruling: "If Humility has the earliest timestamp, then
/// everything is 4/4."
///
/// Re-derived from the current CR: in 7b Humility sets all three 1/1 first,
/// then each Opalescence sets the other two 4/4 [CR#613.6,613.7].
#[test]
fn humility_earliest_among_two_opalescences_makes_all_4_4() {
    let (view, humility, first, second) =
        humility_and_two_opalescences(&["Humility", "Opalescence", "Opalescence"]);
    assert_blank_animated(&view, humility, 4, "Humility");
    assert_blank_animated(&view, first, 4, "the first Opalescence");
    assert_blank_animated(&view, second, 4, "the second Opalescence");
}

/// [CR#613.6]: Dryad of the Ilysian Grove's land-type effect applies in layer
/// 4 [CR#613.1d], before Humility removes the Dryad's abilities in layer 6
/// [CR#613.1f], so the lands keep every basic land type while the Dryad is a
/// 1/1 with no abilities.
#[test]
fn dryad_land_types_survive_humility() {
    let mut state = game_with_p0_deck(vec![
        dryad_of_the_ilysian_grove(),
        card("Humility"),
        card("Kabira Crossroads"),
    ]);
    let ids = enter_in_order(
        &mut state,
        &[
            "Kabira Crossroads",
            "Dryad of the Ilysian Grove",
            "Humility",
        ],
    );
    let [land, dryad, _humility] = ids[..] else { unreachable!() };

    let view = state.layers();
    assert!(
        view.get(dryad).abilities.is_empty(),
        "the Dryad has no abilities"
    );
    assert_eq!(
        (view.power(dryad), view.toughness(dryad)),
        (Some(1), Some(1)),
        "the Dryad is 1/1"
    );
    let subtypes: Vec<&str> = view
        .get(land)
        .subtypes
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    assert_eq!(
        subtypes, BASIC_LAND_TYPES,
        "the land is every basic land type"
    );
}

/// [CR#613.8a,613.8b]: two Humilities animated by Opalescence each remove the
/// other's ability in layer 6, so each depends on the other [CR#613.8a]: a
/// dependency loop, applied in timestamp order among the layer's effects
/// [CR#613.8b]. The first Humility applies before Battle Mastery's later,
/// independent grant, so the first Humility keeps the double strike Battle
/// Mastery gives it. The second Humility's ability is gone before its effect
/// started applying, so that effect no longer exists [CR#604.2] and does not
/// set 1/1 in layer 7b, where the first Humility's 1/1 and then
/// Opalescence's 4/4 apply [CR#613.6,613.7].
#[test]
fn humility_loop_applies_in_timestamp_order() {
    use sylex_core::Ability;
    use sylex_core::KeywordAbility;

    let mut state = game_with_p0_deck(vec![
        card("Humility"),
        opalescence(),
        card("Humility"),
        battle_mastery(),
    ]);
    let ids = enter_in_order(
        &mut state,
        &["Humility", "Opalescence", "Humility", "Battle Mastery"],
    );
    let [first, _opal, second, mastery] = ids[..] else { unreachable!() };
    state.objects.obj_mut(mastery).attached_to = Some(first);

    let view = state.layers();
    assert_eq!(
        view.get(first).abilities.as_slice(),
        [Ability::Keyword(KeywordAbility::DoubleStrike)],
        "the first Humility loses its own ability, then gains double strike"
    );
    assert_eq!(
        (view.power(first), view.toughness(first)),
        (Some(4), Some(4))
    );
    assert_blank_animated(&view, second, 4, "the second Humility");
}

// ---------------------------------------------------------------------------
// Counts in a distributed static ([CR#613.4c])
// ---------------------------------------------------------------------------

/// A semantic card read and lowered through the production boundary
/// (`Plugin::card_from_str`), so its regions carry the register layout
/// lowering produces.
fn lowered(source: &str) -> Arc<Card> {
    Arc::new(
        plugin("canon")
            .card_from_str(source)
            .unwrap_or_else(|error| panic!("fixture did not load: {error}\n{source}"))
            .core,
    )
}

/// Heroes' Podium ({5} Legendary Artifact): "Each legendary creature you
/// control gets +1/+1 for each other legendary creature you control." Its
/// look-at-the-top ability and its own legendary supertype are left off (it is
/// no creature, so no filter here reads it).
///
/// A core fixture with the register layout lowering gives a count filter
/// nested in an `Each` body: the filter's candidate, then the body's
/// candidate captured (`Capture(0)`), then the re-declared source and
/// controller. "Other" excludes the captured candidate, the creature being
/// modified. The v1 semantic language cannot spell that exclusion: `It` and
/// `That(Creature)` inside the count filter both lower to the filter's own
/// candidate.
fn heroes_podium() -> Arc<Card> {
    use sylex_core::CharacteristicPredicate;
    use sylex_core::Count;
    use sylex_core::Countable;
    use sylex_core::DefId;
    use sylex_core::Domain;
    use sylex_core::Kind;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Param;
    use sylex_core::Predicate;
    use sylex_core::Provenance;
    use sylex_core::RefId;
    use sylex_core::Reference;
    use sylex_core::RelationPredicate;
    use sylex_core::StaticSpec;
    use sylex_core::Supertype;
    use sylex_core::Type;
    let entity = |def: u32, provenance: Provenance| Param {
        def: DefId(def),
        kind: Kind::Entity,
        provenance,
    };
    let reg = |r: u32| Arc::new(Predicate::Ref(Reference::Reg(RefId(r))));
    let legendary_creature_controlled_by = |you: u32| {
        vec![
            Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Legendary)),
            Predicate::r#type(Type::Creature),
            Predicate::Relation(RelationPredicate::ControlledBy(reg(you))),
        ]
    };
    let mut other = legendary_creature_controlled_by(3);
    other.push(Predicate::Not(reg(1)));
    let other_legends = Count::CountOf(Countable::Objects(Arc::new(sylex_core::Region::new(
        Arc::from([
            entity(0, Provenance::Candidate(Domain::Object)),
            entity(1, Provenance::Capture(RefId(0))),
            entity(2, Provenance::Source),
            entity(3, Provenance::Controller),
        ]),
        Predicate::And(other.into()),
    ))));
    let body = sylex_core::Region::new(
        Arc::from([
            entity(0, Provenance::Candidate(Domain::Object)),
            entity(1, Provenance::Source),
            entity(2, Provenance::Controller),
        ]),
        StaticSpec::Modify(
            Reference::Reg(RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(other_legends.clone())),
                    Modification::Toughness(NumericOp::Up(other_legends)),
                ]
                .into(),
            ),
        ),
    );
    let pump = sylex_core::Ability::r#static(StaticSpec::Each(
        sylex_core::Selection::SelectAll(static_candidate_filter(Predicate::And(
            legendary_creature_controlled_by(2).into(),
        ))),
        Arc::new(body),
    ));
    fixture(
        "Heroes' Podium",
        vec![sylex_core::ManaSymbol::Simple(
            sylex_core::SimpleManaSymbol::Generic(5),
        )],
        &[Type::Artifact],
        &[],
        None,
        vec![pump],
    )
}

/// Isamaru, Hound of Konda ({W} Legendary Creature — Dog, 2/2). As in every
/// lowered fixture here, a creature type is spelled only where a filter reads
/// it.
fn isamaru() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Isamaru, Hound of Konda",
  mana_cost: [White],
  supertypes: [Legendary],
  types: [Creature],
  power: 2,
  toughness: 2,
)"#,
    )
}

/// Tobias Andrion ({3}{W}{U} Legendary Creature — Human Advisor, 4/4).
fn tobias_andrion() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Tobias Andrion",
  mana_cost: [Generic(3), White, Blue],
  supertypes: [Legendary],
  types: [Creature],
  power: 4,
  toughness: 4,
)"#,
    )
}

/// Joraga Warcaller ({G} Creature — Elf Warrior, 1/1): "Other Elf creatures
/// you control get +1/+1 for each +1/+1 counter on this creature." Its
/// multikicker and enters-with-counters abilities are left off; the test
/// places the counters.
fn joraga_warcaller() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Joraga Warcaller",
  mana_cost: [Green],
  types: [Creature],
  subtypes: [Elf],
  abilities: [
    Static(Each(
      SelectAll(And([Subtype(Elf), Creature, ControlledBy(Ref(You)), Not(Ref(This))])),
      Modify(It, PowerAndToughnessBoth(Up(CounterCount(This, P1P1Counter)))),
    )),
  ],
  power: 1,
  toughness: 1,
)"#,
    )
}

/// Llanowar Elves ({G} Creature — Elf Druid, 1/1). Its mana ability is left
/// off.
fn llanowar_elves() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Llanowar Elves",
  mana_cost: [Green],
  types: [Creature],
  subtypes: [Elf],
  power: 1,
  toughness: 1,
)"#,
    )
}

/// Heroes' Podium's count reads, for each legendary creature it modifies, the
/// legendary creatures you control OTHER than that one [CR#613.4c]: with two
/// legendary creatures each gets +1/+1, not +2/+2 from counting itself. The
/// nonlegendary Grizzly Bears is neither modified nor counted.
#[test]
fn heroes_podium_counts_other_legends_than_the_one_it_modifies() {
    let mut state = game_with_p0_deck(vec![
        heroes_podium(),
        isamaru(),
        tobias_andrion(),
        card("Grizzly Bears"),
    ]);
    let ids = enter_in_order(
        &mut state,
        &[
            "Heroes' Podium",
            "Isamaru, Hound of Konda",
            "Tobias Andrion",
            "Grizzly Bears",
        ],
    );
    let [_podium, isamaru, tobias, bears] = ids[..] else { unreachable!() };

    let view = state.layers();
    assert_eq!(
        (view.power(isamaru), view.toughness(isamaru)),
        (Some(3), Some(3))
    );
    assert_eq!(
        (view.power(tobias), view.toughness(tobias)),
        (Some(5), Some(5))
    );
    assert_eq!(
        (view.power(bears), view.toughness(bears)),
        (Some(2), Some(2))
    );
}

/// Joraga Warcaller's count names the static's source ("this creature")
/// inside the `Each` body, whose register 1 lowering declares as the source,
/// not the controller [CR#613.4c,122.1]: two +1/+1 counters on the Warcaller
/// give the other Elf +2/+2.
#[test]
fn joraga_warcaller_counts_counters_on_itself() {
    use sylex_core::Ability;
    use sylex_core::Ident;
    use sylex_core::Provenance;
    use sylex_core::StaticSpec;

    let warcaller = joraga_warcaller();
    let Card::Normal(face) = &*warcaller else { unreachable!() };
    let Ability::Static(region) = &face.characteristics.abilities[0] else {
        panic!("Joraga Warcaller's ability is a static")
    };
    let StaticSpec::Each(_, body) = &region.body else {
        panic!("Joraga Warcaller's static distributes")
    };
    assert_eq!(
        body.params[1].provenance,
        Provenance::Source,
        "the Each body's register 1 is the static's source"
    );

    let mut state = game_with_p0_deck(vec![warcaller, llanowar_elves()]);
    let ids = enter_in_order(&mut state, &["Joraga Warcaller", "Llanowar Elves"]);
    let [warcaller, elves] = ids[..] else { unreachable!() };
    state
        .objects
        .obj_mut(warcaller)
        .counters
        .insert(Ident::from("P1P1Counter"), 2);

    let view = state.layers();
    assert_eq!(
        (view.power(elves), view.toughness(elves)),
        (Some(3), Some(3))
    );
}

// ---------------------------------------------------------------------------
// "As long as you control …" gates ([CR#611.3a])
// ---------------------------------------------------------------------------

/// Scrapyard Mongrel ({3}{R} Creature — Dog, 3/3): "As long as you control an
/// artifact, this creature gets +2/+0 and has trample." Lowered from the
/// wizards corpus's own spelling, through the `YouControl` macro.
fn scrapyard_mongrel() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Scrapyard Mongrel",
  mana_cost: [Generic(3), Red],
  types: [Creature],
  abilities: [
    Static(Conditionally(YouControl(Type(Artifact)), Modify(This, Several([Power(Up(2)), Toughness(Up(0)), GainAbility(Keyword(Trample))])))),
  ],
  power: 3,
  toughness: 3,
)"#,
    )
}

/// Darksteel Relic ({0} Artifact). Its indestructible is left off.
fn darksteel_relic() -> Arc<Card> {
    lowered(
        r#"Normal(
  name: "Darksteel Relic",
  mana_cost: [Generic(0)],
  types: [Artifact],
)"#,
    )
}

/// Place P0's two cards named `name`, wherever setup dealt them, one in their
/// hand and one in their library.
fn one_in_hand_one_in_library(state: &mut GameState, name: &str) {
    let named = |state: &GameState, o: &ObjectId| is_named(state, *o, name);
    let mut found: Vec<ObjectId> = state.zones.hands[0]
        .iter()
        .chain(state.zones.libraries[0].iter())
        .filter(|o| named(state, o))
        .copied()
        .collect();
    assert_eq!(found.len(), 2, "P0 has two {name} in hand and library");
    state.zones.hands[0].retain(|o| !found.contains(o));
    state.zones.libraries[0].retain(|o| !found.contains(o));
    let in_library = found.pop().expect("two found");
    let in_hand = found.pop().expect("two found");
    state.objects.obj_mut(in_hand).zone = Some(Zone::Hand);
    state.zones.hands[0].push(in_hand);
    state.objects.obj_mut(in_library).zone = Some(Zone::Library);
    state.zones.libraries[0].push_back(in_library);
}

/// Whether Scrapyard Mongrel is the 5/3 trampler its gate grants.
fn mongrel_pumped(view: &sylex_engine::LayeredView, mongrel: ObjectId) -> bool {
    let pumped = (view.power(mongrel), view.toughness(mongrel)) == (Some(5), Some(3));
    let trample = view
        .get(mongrel)
        .abilities
        .contains(&sylex_core::Ability::Keyword(
            sylex_core::KeywordAbility::Trample,
        ));
    assert_eq!(
        pumped, trample,
        "the gate grants +2/+0 and trample together"
    );
    pumped
}

/// An artifact permanent you control opens Scrapyard Mongrel's gate
/// [CR#611.3a,109.2].
#[test]
fn scrapyard_mongrel_with_an_artifact_on_the_battlefield() {
    let mut state = game_with_p0_deck(vec![scrapyard_mongrel(), darksteel_relic()]);
    let ids = enter_in_order(&mut state, &["Scrapyard Mongrel", "Darksteel Relic"]);
    let mongrel = ids[0];
    assert!(mongrel_pumped(&state.layers(), mongrel));
}

/// "You control an artifact" means an artifact permanent [CR#109.2]: artifact
/// cards in your library and hand, which you own but which are not
/// permanents, leave Scrapyard Mongrel a 3/3 without trample.
#[test]
fn scrapyard_mongrel_with_artifacts_only_in_library_and_hand() {
    let mut state = game_with_p0_deck(vec![
        scrapyard_mongrel(),
        darksteel_relic(),
        darksteel_relic(),
    ]);
    let ids = enter_in_order(&mut state, &["Scrapyard Mongrel"]);
    let mongrel = ids[0];
    one_in_hand_one_in_library(&mut state, "Darksteel Relic");
    assert!(!mongrel_pumped(&state.layers(), mongrel));
}

// ---------------------------------------------------------------------------
// Characteristic-defining abilities ([CR#604.3,613.3,613.4a])
// ---------------------------------------------------------------------------

/// A characteristic-defining static making `change` to the object it is on.
fn cda_static(change: sylex_core::Modification) -> sylex_core::Ability {
    sylex_core::Ability::r#static(sylex_core::StaticSpec::CharacteristicDefining(change))
}

/// A creature card whose power and toughness are each `*`, defined by its
/// `abilities` [CR#208.2a].
fn star_creature(
    name: &str,
    mana_cost: Vec<sylex_core::ManaSymbol>,
    subtype: &str,
    abilities: Vec<sylex_core::Ability>,
) -> Arc<Card> {
    use sylex_core::StatValue;
    Arc::new(Card::Normal(sylex_card::CardFace::from(
        sylex_card::Characteristics {
            name: name.into(),
            mana_cost: Arc::<[sylex_core::ManaSymbol]>::from(mana_cost).into(),
            types: vec![sylex_core::Type::Creature.def()],
            subtypes: vec![sylex_core::Subtype {
                name: subtype.into(),
                types: Vec::new().into(),
                confers: Vec::new().into(),
            }],
            power: Some(StatValue::DefinedByAbility),
            toughness: Some(StatValue::DefinedByAbility),
            abilities,
            ..sylex_card::Characteristics::default()
        },
    )))
}

/// `{generic}` followed by `green` green pips.
fn green_cost(generic: u32, green: usize) -> Vec<sylex_core::ManaSymbol> {
    let mut cost = vec![sylex_core::ManaSymbol::Simple(
        sylex_core::SimpleManaSymbol::Generic(generic),
    )];
    cost.extend(std::iter::repeat_n(
        sylex_core::ManaSymbol::from(Color::Green),
        green,
    ));
    cost
}

/// Tarmogoyf ({1}{G} Creature — Lhurgoyf, */1+*): "Tarmogoyf's power is equal
/// to the number of card types among cards in all graveyards and its
/// toughness is equal to that number plus 1."
fn tarmogoyf() -> Arc<Card> {
    use sylex_core::Characteristic;
    use sylex_core::Count;
    use sylex_core::Countable;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Predicate;
    use sylex_core::StatValue;
    use sylex_core::StatePredicate;
    let card_types_in_graveyards = Count::CountDistinct(
        Characteristic::Types,
        Countable::Objects(Arc::new(sylex_core::Region::candidate(Predicate::State(
            StatePredicate::InZone(Zone::Graveyard),
        )))),
    );
    star_creature(
        "Tarmogoyf",
        green_cost(1, 1),
        "Lhurgoyf",
        vec![cda_static(Modification::Several(
            vec![
                Modification::Power(NumericOp::Set(StatValue::Count(
                    card_types_in_graveyards.clone(),
                ))),
                Modification::Toughness(NumericOp::Set(StatValue::Count(Count::Plus(
                    Arc::new(card_types_in_graveyards),
                    Arc::new(Count::Literal(1)),
                )))),
            ]
            .into(),
        ))],
    )
}

/// Maro ({2}{G}{G} Creature — Elemental, */*): "Maro's power and toughness are
/// each equal to the number of cards in your hand."
fn maro() -> Arc<Card> {
    use sylex_core::Count;
    use sylex_core::Countable;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Predicate;
    use sylex_core::Reference;
    use sylex_core::RelationPredicate;
    use sylex_core::StatValue;
    use sylex_core::StatePredicate;
    let cards_in_your_hand =
        Count::CountOf(Countable::Objects(static_candidate_filter(Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Hand)),
                Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
                    Reference::Reg(sylex_core::RefId(2)),
                )))),
            ]
            .into(),
        ))));
    star_creature(
        "Maro",
        green_cost(2, 2),
        "Elemental",
        vec![cda_static(Modification::Several(
            vec![
                Modification::Power(NumericOp::Set(StatValue::Count(cards_in_your_hand.clone()))),
                Modification::Toughness(NumericOp::Set(StatValue::Count(cards_in_your_hand))),
            ]
            .into(),
        ))],
    )
}

/// Godhead of Awe ({W/U}{W/U}{W/U}{W/U}{W/U} Creature — Spirit Avatar, 4/4):
/// "Flying. Other creatures have base power and toughness 1/1." Its flying is
/// left off: these tests read only power and toughness.
fn godhead_of_awe() -> Arc<Card> {
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Predicate;
    use sylex_core::Reference;
    use sylex_core::StatValue;
    use sylex_core::Type;
    let other_creatures = Predicate::And(
        vec![
            Predicate::r#type(Type::Creature),
            Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(sylex_core::RefId(
                1,
            ))))),
        ]
        .into(),
    );
    let hybrid = sylex_core::ManaSymbol::Hybrid(
        sylex_core::SimpleManaSymbol::from(Color::White),
        Color::Blue,
    );
    fixture(
        "Godhead of Awe",
        vec![hybrid; 5],
        &[Type::Creature],
        &["Spirit", "Avatar"],
        Some((4, 4)),
        vec![each_static(
            other_creatures,
            vec![
                Modification::Power(NumericOp::Set(StatValue::Number(1))),
                Modification::Toughness(NumericOp::Set(StatValue::Number(1))),
            ],
        )],
    )
}

/// Transguild Courier ({4} Artifact Creature — Golem, 3/3): "Transguild
/// Courier is all colors."
fn transguild_courier() -> Arc<Card> {
    use sylex_core::CollectionOp;
    use sylex_core::Modification;
    use sylex_core::Type;
    fixture(
        "Transguild Courier",
        vec![sylex_core::ManaSymbol::Simple(
            sylex_core::SimpleManaSymbol::Generic(4),
        )],
        &[Type::Artifact, Type::Creature],
        &["Golem"],
        Some((3, 3)),
        vec![cda_static(Modification::Colors(CollectionOp::Set(
            ALL_COLORS.into(),
        )))],
    )
}

const ALL_COLORS: [Color; 5] = [
    Color::White,
    Color::Blue,
    Color::Black,
    Color::Red,
    Color::Green,
];

/// Put the first card named `name` in P0's hand or library into P0's
/// graveyard.
fn force_into_graveyard(state: &mut GameState, name: &str) -> ObjectId {
    let id = force_onto_battlefield(state, PlayerId(0), name);
    state.zones.battlefield.retain(|&o| o != id);
    state.objects.obj_mut(id).zone = Some(Zone::Graveyard);
    state.zones.graveyards[0].push(id);
    id
}

/// A game with Tarmogoyf in P0's deck beside `others`, a creature card and an
/// instant card in P0's graveyard (two card types), and the cards named in
/// `order` entered in that order.
fn goyf_game(others: Vec<Arc<Card>>, order: &[&str]) -> (GameState, Vec<ObjectId>) {
    let mut deck = vec![tarmogoyf(), card("Lightning Bolt")];
    deck.extend(others);
    let mut state = game_with_p0_deck(deck);
    force_into_graveyard(&mut state, "Grizzly Bears");
    force_into_graveyard(&mut state, "Lightning Bolt");
    let ids = enter_in_order(&mut state, order);
    (state, ids)
}

/// Two card types among cards in graveyards make Tarmogoyf 2/3 [CR#604.3].
#[test]
fn tarmogoyf_counts_card_types_in_graveyards() {
    let (state, ids) = goyf_game(vec![], &["Tarmogoyf"]);
    let view = state.layers();
    assert_eq!(
        (view.power(ids[0]), view.toughness(ids[0])),
        (Some(2), Some(3))
    );
}

/// Glorious Anthem's +1/+1 applies in layer 7c, after Tarmogoyf's own
/// definition in layer 7a [CR#613.4a,613.4c]: 3/4.
#[test]
fn tarmogoyf_under_glorious_anthem_is_its_count_plus_one() {
    let (state, ids) = goyf_game(
        vec![card("Glorious Anthem")],
        &["Glorious Anthem", "Tarmogoyf"],
    );
    let goyf = ids[1];
    let view = state.layers();
    assert_eq!((view.power(goyf), view.toughness(goyf)), (Some(3), Some(4)));
}

/// Turn to Frog ({1}{U} Instant): "Until end of turn, target creature loses
/// all abilities and becomes a blue Frog with base power and toughness 1/1."
/// Tarmogoyf loses its defining ability in layer 6, so nothing defines its
/// power and toughness in layer 7a, and Turn to Frog sets 1/1 in layer 7b
/// [CR#613.1f,613.4a,613.4b].
#[test]
fn tarmogoyf_hit_by_turn_to_frog_is_1_1() {
    use sylex_core::CollectionOp;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::StatValue;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;

    let (mut state, ids) = goyf_game(vec![], &["Tarmogoyf"]);
    let goyf = ids[0];
    state.continuous.push(ContinuousEffect {
        timestamp: sylex_engine::Timestamp(20_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![goyf]),
        changes: vec![
            Modification::LoseAllAbilities,
            Modification::Colors(CollectionOp::Set([Color::Blue].into())),
            Modification::Subtypes(CollectionOp::Set(
                [sylex_core::SubtypeRef::named("Frog".into())].into(),
            )),
            Modification::Power(NumericOp::Set(StatValue::Number(1))),
            Modification::Toughness(NumericOp::Set(StatValue::Number(1))),
        ],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    let view = state.layers();
    assert!(
        view.get(goyf).abilities.is_empty(),
        "Tarmogoyf has no abilities"
    );
    assert_eq!((view.power(goyf), view.toughness(goyf)), (Some(1), Some(1)));
}

/// Godhead of Awe entered before Tarmogoyf, yet its "base power and toughness
/// 1/1" applies in layer 7b, after Tarmogoyf's definition in layer 7a
/// [CR#613.4a,613.4b]: Tarmogoyf is 1/1. Godhead of Awe's 2009-10-01 ruling:
/// its ability "overwrites other effects that set a creature's power and
/// toughness to specific values only if those effects existed before the
/// Godhead entered"; a characteristic-defining ability is not such an effect.
#[test]
fn godhead_of_awe_entered_earlier_still_overwrites_tarmogoyf() {
    let (state, ids) = goyf_game(vec![godhead_of_awe()], &["Godhead of Awe", "Tarmogoyf"]);
    let [godhead, goyf] = ids[..] else { unreachable!() };
    let view = state.layers();
    assert_eq!((view.power(goyf), view.toughness(goyf)), (Some(1), Some(1)));
    assert_eq!(
        (view.power(godhead), view.toughness(godhead)),
        (Some(4), Some(4)),
        "Godhead of Awe affects only other creatures"
    );
}

/// A game with Maro, Glorious Anthem and Godhead of Awe in P0's deck, the
/// cards named in `order` entered in that order, and the number of cards then
/// left in P0's hand.
fn maro_game(order: &[&str]) -> (GameState, Vec<ObjectId>, i32) {
    let mut state = game_with_p0_deck(vec![maro(), card("Glorious Anthem"), godhead_of_awe()]);
    let ids = enter_in_order(&mut state, order);
    let hand = i32::try_from(state.zones.hands[0].len()).expect("hand size fits i32");
    assert!(
        hand >= 2,
        "P0 holds at least two cards, so Maro's count is not 1"
    );
    (state, ids, hand)
}

/// Maro counts the cards in its controller's hand [CR#604.3], and Glorious
/// Anthem's +1/+1 applies after it [CR#613.4a,613.4c].
#[test]
fn maro_under_glorious_anthem_is_its_hand_plus_one() {
    let (state, ids, hand) = maro_game(&["Glorious Anthem", "Maro"]);
    let maro = ids[1];
    let view = state.layers();
    assert_eq!(
        (view.power(maro), view.toughness(maro)),
        (Some(hand + 1), Some(hand + 1))
    );
}

/// Godhead of Awe entered before Maro, yet sets 1/1 in layer 7b after Maro's
/// definition in layer 7a; Glorious Anthem then adds +1/+1 in layer 7c
/// [CR#613.4a..613.4c].
#[test]
fn godhead_of_awe_entered_earlier_still_overwrites_maro() {
    let (state, ids, _) = maro_game(&["Godhead of Awe", "Glorious Anthem", "Maro"]);
    let maro = ids[2];
    let view = state.layers();
    assert_eq!((view.power(maro), view.toughness(maro)), (Some(2), Some(2)));
}

/// Darkest Hour entered before Transguild Courier, yet the Courier's "is all
/// colors" applies first within layer 5 [CR#613.3], so Darkest Hour's "all
/// creatures are black" then leaves it black only.
#[test]
fn transguild_courier_color_definition_applies_first_in_layer_5() {
    let mut state = game_with_p0_deck(vec![card("Darkest Hour"), transguild_courier()]);
    let ids = enter_in_order(&mut state, &["Darkest Hour", "Transguild Courier"]);
    let courier = ids[1];
    assert_eq!(
        state.layers().get(courier).colors.as_slice(),
        [Color::Black]
    );
}

/// With no other color effect, Transguild Courier is all colors [CR#604.3].
#[test]
fn transguild_courier_is_all_colors() {
    let mut state = game_with_p0_deck(vec![transguild_courier()]);
    let ids = enter_in_order(&mut state, &["Transguild Courier"]);
    assert_eq!(state.layers().get(ids[0]).colors.as_slice(), ALL_COLORS);
}

/// Myth Realized ({W} Enchantment). Its abilities are left off: the test
/// applies its "{W}: Until end of turn, this enchantment becomes a Monk Avatar
/// creature in addition to its other types and gains 'This creature's power
/// and toughness are each equal to the number of lore counters on it.'"
/// directly.
fn myth_realized() -> Arc<Card> {
    fixture(
        "Myth Realized",
        vec![sylex_core::ManaSymbol::from(Color::White)],
        &[sylex_core::Type::Enchantment],
        &[],
        None,
        vec![],
    )
}

/// An ability an object grants itself is not characteristic-defining
/// [CR#604.3a]: Myth Realized's gained "power and toughness are each equal to
/// the number of lore counters on it" sets them in layer 7b in timestamp
/// order, after Godhead of Awe, which entered first [CR#613.4b,613.7].
/// Myth Realized's 2015-02-25 ruling: activating the ability "will override
/// any effects that set its power or toughness to a specific value".
#[test]
fn myth_realized_gained_definition_is_not_characteristic_defining() {
    use sylex_core::Ability;
    use sylex_core::CollectionOp;
    use sylex_core::Count;
    use sylex_core::Duration;
    use sylex_core::Modification;
    use sylex_core::NumericOp;
    use sylex_core::Reference;
    use sylex_core::StatValue;
    use sylex_core::StaticSpec;
    use sylex_engine::ContinuousEffect;
    use sylex_engine::ScopeResolved;

    let mut state = game_with_p0_deck(vec![godhead_of_awe(), myth_realized()]);
    let ids = enter_in_order(&mut state, &["Godhead of Awe", "Myth Realized"]);
    let myth = ids[1];
    state
        .objects
        .obj_mut(myth)
        .counters
        .insert("LoreCounter".into(), 3);
    let lore_counters = Count::CounterCount(
        Arc::new(Reference::Reg(sylex_core::RefId(0))),
        sylex_core::CounterRef("LoreCounter".into()),
    );
    let gained = Ability::r#static(StaticSpec::CharacteristicDefining(Modification::Several(
        vec![
            Modification::Power(NumericOp::Set(StatValue::Count(lore_counters.clone()))),
            Modification::Toughness(NumericOp::Set(StatValue::Count(lore_counters))),
        ]
        .into(),
    )));
    state.continuous.push(ContinuousEffect {
        timestamp: sylex_engine::Timestamp(20_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![myth]),
        changes: vec![
            Modification::CardTypes(CollectionOp::Add(sylex_core::Type::Creature.name())),
            Modification::Subtypes(CollectionOp::Add(sylex_core::SubtypeRef::named(
                "Monk".into(),
            ))),
            Modification::Subtypes(CollectionOp::Add(sylex_core::SubtypeRef::named(
                "Avatar".into(),
            ))),
            Modification::GainAbility(Arc::new(gained)),
        ],
        duration: Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
        rows: vec![],
        origin: None,
        is_cda: false,
    });
    let view = state.layers();
    assert_eq!((view.power(myth), view.toughness(myth)), (Some(3), Some(3)));
}
