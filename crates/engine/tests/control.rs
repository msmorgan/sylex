//! A creature stolen by a continuous control-changing effect, driven through
//! real turns: the rules read its derived controller ([CR#613.1b]) for
//! untapping [CR#502.3], summoning sickness [CR#302.6], triggered-ability
//! control [CR#603.3a] and lifelink [CR#702.15b].
//!
//! The steal is a locked layer-2 `SetController` for the rest of the game,
//! timestamped when it is made: Control Magic's "You control enchanted
//! creature" without the Aura, the same shape as each half of Avarice Totem's
//! exchange. A resolved "gain control" (Wild Dogs) makes the same kind of
//! effect with its own timestamp ([CR#611.2a,613.7b]).

use std::path::Path;
use std::sync::Arc;

use sylex_card::Card;
use sylex_core::Color;
use sylex_core::Duration;
use sylex_core::Modification;
use sylex_core::PhaseStep;
use sylex_core::Reference;
use sylex_core::Zone;
use sylex_engine::Action;
use sylex_engine::ContinuousEffect;
use sylex_engine::Decision;
use sylex_engine::DecisionPointKind;
use sylex_engine::GameConfig;
use sylex_engine::GameState;
use sylex_engine::ManaProvenance;
use sylex_engine::ObjectId;
use sylex_engine::PlayerConfig;
use sylex_engine::PlayerId;
use sylex_engine::ScopeResolved;
use sylex_engine::StartingPlayer;
use sylex_engine::StepOutcome;
use sylex_engine::Timestamp;
use sylex_engine::WorkItem;
use sylex_plugin_v1::plugin::Plugin;

const P0: PlayerId = PlayerId(0);
const P1: PlayerId = PlayerId(1);

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

/// A hand-spelled semantic card derived from a real one, lowered through the
/// production boundary (`Plugin::card_from_str`).
fn fixture(source: &str) -> Arc<Card> {
    Arc::new(
        plugin("canon")
            .card_from_str(source)
            .unwrap_or_else(|error| panic!("fixture did not load: {error}\n{source}"))
            .core,
    )
}

/// Switcheroo: "Exchange control of two target creatures."
const SWITCHEROO: &str = r#"Normal(
    name: "Switcheroo",
    mana_cost: [Generic(4), Blue],
    types: [Sorcery],
    abilities: [
        Spell(effect: Targeted(
            targets: [TargetOne(Creature), TargetOne(Creature)],
            effect: ExchangeControl(Target(0), Target(1)),
        )),
    ],
)"#;

/// A two-player game, P0 first: P0's deck holds `p0_cards` and P1's
/// `p1_cards`, each padded with Grizzly Bears to 30 cards so no library runs
/// out. Builtin SBA and damage-result rules are wired so combat damage and
/// the lethal-damage check behave as in a real game.
fn game(p0_cards: &[&str], p1_cards: &[&str]) -> GameState {
    let cards = |names: &[&str]| names.iter().map(|n| card(n)).collect();
    game_of(cards(p0_cards), cards(p1_cards))
}

/// [`game`] with the decks' cards given directly.
fn game_of(p0_cards: Vec<Arc<Card>>, p1_cards: Vec<Arc<Card>>) -> GameState {
    let deck = |mut deck: Vec<Arc<Card>>| {
        while deck.len() < 30 {
            deck.push(card("Grizzly Bears"));
        }
        deck
    };
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(p0_cards),
            },
            PlayerConfig {
                deck: deck(p1_cards),
            },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(P0),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let builtin = plugin("builtin");
    state.sba_rules = builtin.sba_rules;
    state.damage_result_rules = builtin.damage_result_rules;
    state
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

/// Moves the first `name` card from `player`'s hand or library onto the
/// battlefield, as a permanent `player` has controlled since before the game's
/// first turn.
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let p = player.index();
    let named = |state: &GameState, o: ObjectId| face_name(state, o) == name;
    let obj = if let Some(i) = state.zones.hands[p].iter().position(|&o| named(state, o)) {
        state.zones.hands[p].remove(i)
    } else {
        let i = state.zones.libraries[p]
            .iter()
            .position(|&o| named(state, o))
            .unwrap_or_else(|| panic!("no {name} in P{}'s hand or library", player.0));
        state.zones.libraries[p]
            .remove(i)
            .expect("index in library")
    };
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// `to` gains control of `id` for the rest of the game through a locked
/// layer-2 `SetController` whose `You` is `to` ([CR#611.2c,613.1b]), plus
/// any `also` changes (Act of Treason's haste grant, applied in layer 6
/// [CR#613.6]). Returns the effect's timestamp, drawn from the game's clock
/// [CR#613.7].
fn steal(state: &mut GameState, id: ObjectId, to: PlayerId, also: Vec<Modification>) -> Timestamp {
    steal_for(state, id, to, also, Duration::EndOfGame)
}

/// [`steal`] with a stated `duration` (Threaten's "until end of turn").
fn steal_for(
    state: &mut GameState,
    id: ObjectId,
    to: PlayerId,
    also: Vec<Modification>,
    duration: Duration,
) -> Timestamp {
    let mut changes = vec![Modification::SetController(
        Reference::controller_parameter(),
    )];
    changes.extend(also);
    let timestamp = state.objects.next_timestamp();
    state.continuous.push(ContinuousEffect {
        timestamp,
        controller: to,
        scope: ScopeResolved::Locked(vec![id]),
        changes,
        duration,
        rows: vec![],
        origin: None,
    });
    timestamp
}

/// Steps the game, answering every decision with the do-nothing choice
/// (pass, no attackers, no blockers, discard from the front of the hand,
/// triggers in printed order), until `stop` accepts a decision, which it
/// returns unanswered.
fn drive(
    state: &mut GameState,
    mut stop: impl FnMut(&GameState, &DecisionPointKind) -> bool,
) -> DecisionPointKind {
    for _ in 0..100_000 {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(decision) => {
                if stop(state, &decision) {
                    return decision;
                }
                let answer = match &decision {
                    DecisionPointKind::Priority(_) => Decision::Act(Action::Pass),
                    DecisionPointKind::DeclareAttackers(_) => Decision::Attackers(vec![]),
                    DecisionPointKind::DeclareBlockers(_) => Decision::Blocks(vec![]),
                    DecisionPointKind::DiscardToHandSize(prompt) => Decision::Discard(
                        state.zones.hands[prompt.player.index()]
                            .iter()
                            .copied()
                            .take(usize::try_from(prompt.count).unwrap())
                            .collect(),
                    ),
                    DecisionPointKind::OrderTriggers(prompt) => {
                        Decision::Order((0..prompt.triggers.len()).collect())
                    }
                    other => panic!("unexpected decision while driving: {other:?}"),
                };
                state.submit_decision(answer).unwrap();
            }
            StepOutcome::GameOver(outcome) => panic!("game ended while driving: {outcome:?}"),
        }
    }
    panic!("drive did not reach its stop");
}

/// Drives to `turn`'s declare-attackers decision and returns its legal
/// attackers.
fn attackers_on_turn(state: &mut GameState, turn: u32) -> Vec<ObjectId> {
    match drive(state, |s, d| {
        matches!(d, DecisionPointKind::DeclareAttackers(_)) && s.turn.turn_number == turn
    }) {
        DecisionPointKind::DeclareAttackers(prompt) => prompt.legal,
        other => unreachable!("{other:?}"),
    }
}

/// Drives to `player`'s first priority in `step` of `turn`, returning the
/// legal actions there.
fn priority_on_turn(
    state: &mut GameState,
    turn: u32,
    step: PhaseStep,
    player: PlayerId,
) -> Vec<Action> {
    match drive(state, |s, d| {
        matches!(d, DecisionPointKind::Priority(p) if p.player == player)
            && s.turn.turn_number == turn
            && s.turn.current == step
    }) {
        DecisionPointKind::Priority(prompt) => prompt.legal,
        other => unreachable!("{other:?}"),
    }
}

/// Attacks the sole opponent of the creatures' controller with `attackers`.
fn attack(state: &mut GameState, attackers: &[ObjectId]) {
    let defender = state
        .player(state.next_live_after(state.turn.active_player))
        .object;
    state
        .submit_decision(Decision::Attackers(
            attackers.iter().map(|&a| (a, defender)).collect(),
        ))
        .unwrap();
}

fn activates(legal: &[Action], object: ObjectId) -> bool {
    legal
        .iter()
        .any(|a| matches!(a, Action::ActivateAbility { object: o, .. } if *o == object))
}

/// [CR#502.3,613.1b]: the active player untaps the permanents they control,
/// and that is the derived controller. P0's Grizzly Bears, stolen by P1 and
/// tapped, stays tapped through its owner's untap step and untaps in its new
/// controller's.
#[test]
fn stolen_creature_untaps_in_its_new_controllers_untap_step() {
    let mut state = game(&[], &[]);
    let bear = force_onto_battlefield(&mut state, P0, "Grizzly Bears");
    steal(&mut state, bear, P1, vec![]);
    state.objects.obj_mut(bear).tapped = true;

    let _ = priority_on_turn(
        &mut state,
        1,
        PhaseStep::Beginning(sylex_core::BeginningStep::Upkeep),
        P0,
    );
    assert!(
        state.objects.obj(bear).tapped,
        "its owner's untap step does not untap a creature they no longer control"
    );
    let _ = priority_on_turn(
        &mut state,
        2,
        PhaseStep::Beginning(sylex_core::BeginningStep::Upkeep),
        P1,
    );
    assert!(
        !state.objects.obj(bear).tapped,
        "its new controller's untap step untaps it"
    );
}

/// [CR#302.6]: a creature stolen by a continuous effect during its new
/// controller's turn has not been under their control since that turn began,
/// so it can't attack that turn; it can on their next turn. Its owner can't
/// attack with it in between.
#[test]
fn stolen_creature_attacks_for_its_new_controller_from_their_next_turn() {
    let mut state = game(&[], &[]);
    let bear = force_onto_battlefield(&mut state, P0, "Grizzly Bears");
    assert!(
        attackers_on_turn(&mut state, 1).contains(&bear),
        "sanity: its owner can attack with it before the steal"
    );

    let _ = priority_on_turn(&mut state, 2, PhaseStep::PrecombatMain, P1);
    steal(&mut state, bear, P1, vec![]);
    assert!(
        !attackers_on_turn(&mut state, 2).contains(&bear),
        "stolen this turn and without haste, it can't attack ([CR#302.6])"
    );
    assert!(
        !attackers_on_turn(&mut state, 3).contains(&bear),
        "its owner no longer controls it ([CR#613.1b])"
    );
    assert!(
        attackers_on_turn(&mut state, 4).contains(&bear),
        "under its new controller since their turn began, it can attack"
    );
}

/// [CR#302.6,702.10b,613.6]: Act of Treason's shape (gain control, and the
/// creature gains haste) lets the stolen creature attack at once.
#[test]
fn stolen_creature_with_haste_attacks_at_once() {
    let mut state = game(&[], &[]);
    let bear = force_onto_battlefield(&mut state, P0, "Grizzly Bears");

    let _ = priority_on_turn(&mut state, 2, PhaseStep::PrecombatMain, P1);
    let haste = sylex_core::Ability::Keyword(sylex_core::KeywordAbility::Composite {
        name: "Haste".into(),
        abilities: vec![],
    });
    steal(
        &mut state,
        bear,
        P1,
        vec![Modification::GainAbility(Arc::new(haste))],
    );
    assert!(
        attackers_on_turn(&mut state, 2).contains(&bear),
        "a stolen creature with haste can attack the turn it is stolen"
    );
}

/// [CR#302.6]: a stolen creature's {T} ability (Prodigal Sorcerer's ping, as
/// the testing pinger) can't be activated until its new controller's turn
/// begins with it under their control, and its owner can't activate it at
/// all.
#[test]
fn stolen_creature_tap_ability_waits_for_its_new_controllers_turn() {
    let pinger_name = "Creature tap-activated DealDamage AnyTarget";
    let mut state = game(&[pinger_name], &[]);
    let pinger = force_onto_battlefield(&mut state, P0, pinger_name);
    steal(&mut state, pinger, P1, vec![]);

    let upkeep = PhaseStep::Beginning(sylex_core::BeginningStep::Upkeep);
    assert!(
        !activates(&priority_on_turn(&mut state, 1, upkeep, P0), pinger),
        "its owner no longer controls it ([CR#602.2])"
    );
    assert!(
        !activates(&priority_on_turn(&mut state, 1, upkeep, P1), pinger),
        "not under P1's control since P1's most recent turn began ([CR#302.6])"
    );
    assert!(
        activates(&priority_on_turn(&mut state, 2, upkeep, P1), pinger),
        "P1's turn began with it under P1's control"
    );
}

/// [CR#603.3a]: a triggered ability is controlled by its source's controller
/// when it triggered. P1's stolen Library Larcenist ("Whenever this creature
/// attacks, draw a card.") attacks for P1, and P1 controls the trigger and
/// draws.
#[test]
fn stolen_creatures_trigger_is_controlled_by_its_new_controller() {
    let mut state = game(&["Library Larcenist"], &[]);
    let larcenist = force_onto_battlefield(&mut state, P0, "Library Larcenist");
    steal(&mut state, larcenist, P1, vec![]);

    let legal = attackers_on_turn(&mut state, 2);
    assert!(legal.contains(&larcenist), "sanity: P1 can attack with it");
    attack(&mut state, &[larcenist]);
    let _ = drive(&mut state, |s, d| {
        matches!(d, DecisionPointKind::Priority(_)) && !s.stack.is_empty()
    });
    let trigger = state
        .stack
        .last()
        .expect("the attack trigger is on the stack");
    assert_eq!(
        trigger.controller, P1,
        "the stolen creature's trigger is controlled by its new controller"
    );
    let hand = state.zones.hands[P1.index()].len();
    let _ = drive(&mut state, |s, d| {
        matches!(d, DecisionPointKind::Priority(_)) && s.stack.is_empty()
    });
    assert_eq!(
        state.zones.hands[P1.index()].len(),
        hand + 1,
        "P1, the trigger's controller, draws the card"
    );
}

/// [CR#702.15b]: lifelink damage makes the source's controller gain life.
/// P1's stolen Steadfast Paladin (lifelink) deals 2 combat damage to P0, and
/// P1 gains the life.
#[test]
fn stolen_lifelinks_damage_pays_its_new_controller() {
    let mut state = game(&["Steadfast Paladin"], &[]);
    let paladin = force_onto_battlefield(&mut state, P0, "Steadfast Paladin");
    steal(&mut state, paladin, P1, vec![]);

    let legal = attackers_on_turn(&mut state, 2);
    assert!(legal.contains(&paladin), "sanity: P1 can attack with it");
    attack(&mut state, &[paladin]);
    let _ = priority_on_turn(&mut state, 2, PhaseStep::PostcombatMain, P1);
    assert_eq!(state.player(P0).life, 18, "the Paladin hit P0 for 2");
    assert_eq!(
        state.player(P1).life,
        22,
        "lifelink pays its new controller ([CR#702.15b])"
    );
}

/// [CR#613.1b]: Glorious Anthem's "Creatures you control get +1/+1." (layer
/// 7c) matches what its controller controls after layer 2: P1's Anthem pumps
/// the Grizzly Bears P1 stole, and P0's Anthem no longer does.
#[test]
fn creatures_you_control_matches_a_stolen_creature() {
    let mut state = game(&["Glorious Anthem"], &["Glorious Anthem"]);
    let bear = force_onto_battlefield(&mut state, P0, "Grizzly Bears");
    let p1_anthem = force_onto_battlefield(&mut state, P1, "Glorious Anthem");
    assert_eq!(
        state.layers().power(bear),
        Some(2),
        "sanity: P1's Anthem does not pump P0's Bears"
    );
    steal(&mut state, bear, P1, vec![]);
    assert_eq!(
        state.layers().power(bear),
        Some(3),
        "P1's Anthem pumps the Bears P1 now controls"
    );

    state.zones.battlefield.retain(|&o| o != p1_anthem);
    state.objects.obj_mut(p1_anthem).zone = Some(Zone::Graveyard);
    let _p0_anthem = force_onto_battlefield(&mut state, P0, "Glorious Anthem");
    assert_eq!(
        state.layers().power(bear),
        Some(2),
        "P0's Anthem no longer matches the Bears P0 lost control of"
    );
}

/// [CR#109.5,613.1b]: a static ability's "you" is the current controller of
/// the permanent it is on. P0's Glorious Anthem, stolen by P1, pumps P1's
/// Grizzly Bears and no longer pumps P0's.
#[test]
fn stolen_anthem_pumps_its_new_controllers_creatures() {
    let mut state = game(&["Glorious Anthem"], &[]);
    let anthem = force_onto_battlefield(&mut state, P0, "Glorious Anthem");
    let p0_bear = force_onto_battlefield(&mut state, P0, "Grizzly Bears");
    let p1_bear = force_onto_battlefield(&mut state, P1, "Grizzly Bears");
    assert_eq!(
        state.layers().power(p0_bear),
        Some(3),
        "sanity: P0's Anthem pumps P0's Bears"
    );
    steal(&mut state, anthem, P1, vec![]);
    let view = state.layers();
    assert_eq!(
        view.power(p1_bear),
        Some(3),
        "the stolen Anthem pumps its new controller's Bears"
    );
    assert_eq!(
        view.power(p0_bear),
        Some(2),
        "and no longer pumps its owner's"
    );
}

/// [CR#302.6]: control returning to the owner is a control change too. P1
/// steals P0's pinger (Prodigal Sorcerer's "{T}: This creature deals 1 damage
/// to any target.") until end of turn during P0's turn 1, as Threaten would.
/// When the effect ends at cleanup, P0 has not controlled it continuously since
/// P0's most recent turn began, so P0 can't activate its {T} ability during
/// P1's turn 2, and can once P0's turn 3 begins.
#[test]
fn creature_returned_by_an_ended_steal_is_sick_for_its_owner() {
    let pinger_name = "Creature tap-activated DealDamage AnyTarget";
    let mut state = game(&[pinger_name], &[]);
    let pinger = force_onto_battlefield(&mut state, P0, pinger_name);
    let upkeep = PhaseStep::Beginning(sylex_core::BeginningStep::Upkeep);

    assert!(
        activates(&priority_on_turn(&mut state, 1, upkeep, P0), pinger),
        "sanity: its owner can activate it before the steal"
    );
    steal_for(
        &mut state,
        pinger,
        P1,
        vec![],
        Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
    );
    let _ = priority_on_turn(
        &mut state,
        1,
        PhaseStep::Ending(sylex_core::EndingStep::End),
        P0,
    );
    assert_eq!(
        state.layers().controller(pinger),
        P1,
        "sanity: still stolen"
    );

    let legal = priority_on_turn(&mut state, 2, upkeep, P0);
    assert_eq!(
        state.layers().controller(pinger),
        P0,
        "the steal ended at cleanup"
    );
    assert!(
        !activates(&legal, pinger),
        "back under P0's control only since P0's turn ended ([CR#302.6])"
    );
    assert!(
        activates(&priority_on_turn(&mut state, 3, upkeep, P0), pinger),
        "P0's turn 3 began with it under P0's control"
    );
}

/// [CR#611.2a,613.7]: a resolved "gain control" is a layer-2 effect with its
/// own timestamp, so it applies after an earlier control effect. P1's Control
/// Magic holds P0's Wild Dogs ("At the beginning of your upkeep, if a player
/// has more life than each other player, the player with the most life gains
/// control of this creature."). At P1's upkeep P0 has the most life and gains
/// control of the Dogs, though Control Magic still applies; P0 gained it on
/// another player's turn, so it is summoning sick for P0 [CR#302.6].
#[test]
fn a_later_gain_control_overrides_control_magic() {
    let mut state = game(&["Wild Dogs"], &[]);
    let dogs = force_onto_battlefield(&mut state, P0, "Wild Dogs");
    let _control_magic = steal(&mut state, dogs, P1, vec![]);
    state.player_mut(P0).life = 25;

    let _ = priority_on_turn(&mut state, 2, PhaseStep::PrecombatMain, P1);
    assert_eq!(
        state.layers().controller(dogs),
        P0,
        "the gain-control effect is later than Control Magic's and wins"
    );
    assert_eq!(
        state.objects.obj(dogs).controller,
        P0,
        "the base controller stays the one it entered under ([CR#110.2])"
    );
    assert!(
        state.objects.obj(dogs).summoning_sick,
        "P0 has not controlled it since P0's most recent turn began"
    );
}

/// [CR#611.2a]: gaining control of a permanent its new controller already
/// controls still creates an effect that lasts until the end of the game.
/// P1's Control Magic holds P0's Wild Dogs; at P1's upkeep P1 has the most
/// life and gains control of the Dogs. When Control Magic's effect ends, P1's
/// gain-control effect still applies and P1 keeps the Dogs. Control never
/// changed, so the Dogs are not newly summoning sick [CR#302.6].
#[test]
fn gain_control_of_a_creature_already_controlled_outlasts_control_magic() {
    let mut state = game(&["Wild Dogs"], &[]);
    let dogs = force_onto_battlefield(&mut state, P0, "Wild Dogs");
    let control_magic = steal(&mut state, dogs, P1, vec![]);
    state.player_mut(P1).life = 30;

    let _ = priority_on_turn(&mut state, 2, PhaseStep::PrecombatMain, P1);
    assert_eq!(state.layers().controller(dogs), P1, "sanity: still P1's");
    assert!(
        !state.objects.obj(dogs).summoning_sick,
        "P1 has controlled it continuously since P1's turn began"
    );
    state.continuous.retain(|e| e.timestamp != control_magic);
    assert_eq!(
        state.layers().controller(dogs),
        P1,
        "P1's own gain-control effect outlasts Control Magic's"
    );
}

/// Moves the first `name` card `player` owns from their library into their
/// hand.
fn into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let p = player.index();
    let i = state.zones.libraries[p]
        .iter()
        .position(|&o| face_name(state, o) == name)
        .unwrap_or_else(|| panic!("no {name} in P{}'s library", player.0));
    let obj = state.zones.libraries[p]
        .remove(i)
        .expect("index in library");
    state.objects.obj_mut(obj).zone = Some(Zone::Hand);
    state.zones.hands[p].push(obj);
    obj
}

/// Floats `amount` `color` mana into `player`'s pool and reopens the priority
/// window in flight so the pool shows in its legal actions.
fn float(state: &mut GameState, player: PlayerId, color: Color, amount: u32) {
    state
        .player_mut(player)
        .mana_pool
        .add(color.into(), amount, ManaProvenance::default());
    assert!(
        matches!(state.pending, Some(DecisionPointKind::Priority(_))),
        "float expects a priority window in flight"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

/// Steps the game, answering each decision with `answer` first and the
/// routine answer otherwise (pass, auto-pay), until `stop` accepts a decision,
/// which it returns unanswered.
fn drive_answering(
    state: &mut GameState,
    mut answer: impl FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
    mut stop: impl FnMut(&GameState, &DecisionPointKind) -> bool,
) -> DecisionPointKind {
    for _ in 0..10_000 {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(decision) => {
                if stop(state, &decision) {
                    return decision;
                }
                let routine = match &decision {
                    DecisionPointKind::Priority(_) => Some(Decision::Act(Action::Pass)),
                    DecisionPointKind::PayMana(_) => Some(Decision::Pay(state.auto_pay_pending())),
                    DecisionPointKind::Payment(_) => state.auto_payment_pending(),
                    _ => None,
                };
                let chosen = answer(state, &decision)
                    .or(routine)
                    .unwrap_or_else(|| panic!("no answer for {decision:?}"));
                state.submit_decision(chosen).unwrap();
            }
            StepOutcome::GameOver(outcome) => panic!("game ended while driving: {outcome:?}"),
        }
    }
    panic!("drive_answering did not reach its stop");
}

/// [CR#603.3a,701.12b]: a triggered ability is controlled by the player who
/// controlled its source when it triggered. P0 casts Switcheroo on P0's
/// Grizzly Bears and P1's Footlight Fiend ("When this creature dies, it deals
/// 1 damage to any target."), so P0 gains control of the Fiend through a
/// resolved effect. P0 then bolts the Fiend, and its dies trigger is P0's.
#[test]
fn a_gained_creatures_dies_trigger_is_controlled_by_the_player_who_gained_it() {
    let mut state = game_of(
        vec![fixture(SWITCHEROO), card("Lightning Bolt")],
        vec![card("Footlight Fiend")],
    );
    let bears = force_onto_battlefield(&mut state, P0, "Grizzly Bears");
    let fiend = force_onto_battlefield(&mut state, P1, "Footlight Fiend");
    let switcheroo = into_hand(&mut state, P0, "Switcheroo");
    let bolt = into_hand(&mut state, P0, "Lightning Bolt");
    let main = |s: &GameState, d: &DecisionPointKind| {
        matches!(d, DecisionPointKind::Priority(p) if p.player == P0)
            && s.turn.turn_number == 1
            && s.turn.current == PhaseStep::PrecombatMain
            && s.stack.is_empty()
    };
    let _ = drive_answering(&mut state, |_, _| None, main);

    float(&mut state, P0, Color::Blue, 5);
    let _ = drive_answering(&mut state, |_, _| None, main);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: switcheroo }))
        .unwrap();
    let _ = drive_answering(
        &mut state,
        |_, d| {
            matches!(d, DecisionPointKind::ChooseTargets(_))
                .then(|| Decision::Targets(vec![vec![bears], vec![fiend]]))
        },
        main,
    );
    assert_eq!(state.layers().controller(fiend), P0, "P0 gained the Fiend");

    float(&mut state, P0, Color::Red, 1);
    let _ = drive_answering(&mut state, |_, _| None, main);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let bolt_targeted = std::cell::Cell::new(false);
    let prompt = drive_answering(
        &mut state,
        |_, d| match d {
            DecisionPointKind::ChooseTargets(_) if !bolt_targeted.get() => {
                bolt_targeted.set(true);
                Some(Decision::Targets(vec![vec![fiend]]))
            }
            _ => None,
        },
        |_, d| bolt_targeted.get() && matches!(d, DecisionPointKind::ChooseTargets(_)),
    );
    let DecisionPointKind::ChooseTargets(prompt) = prompt else {
        unreachable!("{prompt:?}")
    };
    assert_eq!(
        prompt.player, P0,
        "the dies trigger's controller, P0, chooses its target"
    );
    let p1 = state.player(P1).object;
    state
        .submit_decision(Decision::Targets(vec![vec![p1]]))
        .unwrap();
    let _ = drive_answering(
        &mut state,
        |_, _| None,
        |s, d| matches!(d, DecisionPointKind::Priority(_)) && !s.stack.is_empty(),
    );
    let trigger = state
        .stack
        .last()
        .expect("the dies trigger is on the stack");
    assert_eq!(
        trigger.controller, P0,
        "the Fiend's dies trigger is controlled by the player who gained it"
    );
}
