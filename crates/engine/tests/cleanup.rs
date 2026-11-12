//! The cleanup step ([CR#514]) against real canon cards: the hand-size discard
//! comes first, damage removal and the end of "this turn" effects second, and
//! a state-based action or a waiting trigger gives the active player priority
//! and another cleanup step.

use std::path::Path;
use std::sync::Arc;

use sylex_card::Card;
use sylex_core::Duration;
use sylex_core::EndingStep;
use sylex_core::EventFilter;
use sylex_core::Instruction;
use sylex_core::PhaseStep;
use sylex_core::Predicate;
use sylex_core::Reference;
use sylex_core::Replacement;
use sylex_core::TurnMarker;
use sylex_core::Zone;
use sylex_engine::Action;
use sylex_engine::Decision;
use sylex_engine::DecisionPointKind;
use sylex_engine::GameConfig;
use sylex_engine::GameEvent;
use sylex_engine::GameState;
use sylex_engine::InstanceId;
use sylex_engine::ObjectId;
use sylex_engine::Occurrence;
use sylex_engine::PlayerConfig;
use sylex_engine::PlayerId;
use sylex_engine::Progress;
use sylex_engine::ReplacementInstance;
use sylex_engine::StartingPlayer;
use sylex_engine::StepOutcome;
use sylex_plugin_v1::plugin::Plugin;

const CLEANUP: PhaseStep = PhaseStep::Ending(EndingStep::Cleanup);

fn canon_card(name: &str) -> Arc<Card> {
    let canon = Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../v1/plugins/canon"),
    )
    .unwrap();
    Arc::new(canon.card(name).unwrap().core)
}

/// A two-player game in which both players' decks are `name`, player 0 going
/// first. Player 0 starts the game with `extra` more cards in hand, drawn from
/// the top of their library, so turn 1's cleanup has a discard to make.
fn game_with(name: &str, extra: usize) -> GameState {
    let card = canon_card(name);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![Arc::clone(&card); 20],
            },
            PlayerConfig {
                deck: vec![card; 20],
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
    for _ in 0..extra {
        let top = state.zones.libraries[0]
            .pop_front()
            .expect("the library has cards");
        state.objects.obj_mut(top).zone = Some(Zone::Hand);
        state.zones.hands[0].push(top);
    }
    state
}

/// Moves `object` from player 0's hand onto the battlefield.
fn hand_to_battlefield(state: &mut GameState, object: ObjectId) {
    state.zones.hands[0].retain(|&o| o != object);
    state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(object);
}

/// Steps on, passing every priority outside the cleanup step and declaring no
/// attackers, until turn 1's cleanup discard surfaces.
fn drive_to_cleanup_discard(state: &mut GameState) -> (PlayerId, u32) {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
                assert_ne!(
                    state.turn.current, CLEANUP,
                    "no priority in cleanup before the discard"
                );
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DeclareAttackers(_)) => {
                state.submit_decision(Decision::Attackers(vec![])).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DiscardToHandSize(
                sylex_engine::DiscardToHandSize { player, count },
            )) => return (player, count),
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
}

/// Steps on, passing every priority and declaring no attackers, until player
/// 0 holds priority in turn 1's end step.
fn drive_to_end_step(state: &mut GameState) {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
                if state.turn.turn_number == 1
                    && state.turn.current == PhaseStep::Ending(EndingStep::End)
                {
                    return;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DeclareAttackers(_)) => {
                state.submit_decision(Decision::Attackers(vec![])).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
}

/// How many times `trace` shows the cleanup step beginning.
fn cleanup_steps_begun(trace: &[Progress]) -> usize {
    trace
        .iter()
        .filter(|p| **p == Progress::Advanced(CLEANUP))
        .count()
}

/// [CR#514.1,514.3a,702.35a]: Falkenrath Gorger gives each other Vampire
/// creature card its owner owns off the battlefield madness. A Gorger
/// discarded to hand size in cleanup is exiled, its madness trigger waits, so
/// the trigger goes on the stack and the active player gets priority in the
/// cleanup step; once the stack is empty and both players pass, another
/// cleanup step begins, and that one ends the turn.
#[test]
fn a_madness_discard_in_cleanup_gives_priority_and_another_cleanup_step() {
    // Opening seven plus one: one Gorger onto the battlefield leaves eight in
    // hand, so turn 1's cleanup discards one.
    let mut state = game_with("Falkenrath Gorger", 2);
    let gorger = state.zones.hands[0][0];
    hand_to_battlefield(&mut state, gorger);

    let (player, count) = drive_to_cleanup_discard(&mut state);
    assert_eq!((player, count), (PlayerId(0), 1));
    assert_eq!(state.turn.turn_number, 1);
    let discarded = state.zones.hands[0][0];
    state
        .submit_decision(Decision::Discard(vec![discarded]))
        .unwrap();

    let mut trace = Vec::new();
    let mut cleanup_priority_with_trigger = false;
    loop {
        match state.step() {
            StepOutcome::Progress(p) => {
                let next_turn = matches!(
                    p,
                    Progress::Applied(Occurrence::Single(GameEvent::TurnBegan(_)))
                );
                trace.push(p);
                if next_turn {
                    break;
                }
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
                assert_eq!(
                    state.turn.current, CLEANUP,
                    "priority is offered only in the cleanup step"
                );
                if !state.stack.is_empty() {
                    cleanup_priority_with_trigger = true;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            // The madness trigger resolves: decline to cast the Gorger.
            StepOutcome::NeedsDecision(DecisionPointKind::YesNo(_)) => {
                state.submit_decision(Decision::Answer(false)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
    assert!(
        cleanup_priority_with_trigger,
        "the madness trigger went on the stack and the active player got priority \
         in cleanup ([CR#514.3a])"
    );
    assert_eq!(
        cleanup_steps_begun(&trace),
        1,
        "another cleanup step began after the stack emptied ([CR#514.3a]); trace: {trace:?}"
    );
    assert_eq!(
        state.turn.turn_number, 2,
        "the second cleanup step ended the turn"
    );
    assert_eq!(state.zones.hands[0].len(), 7);
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the declined madness card went to the graveyard ([CR#702.35a])"
    );
}

/// [CR#514.1,514.2]: the discard to hand size comes before "this turn" effects
/// end, so a "this turn" replacement still applies to it. The replacement is
/// Yawgmoth's Will's "If a card would be put into your graveyard from anywhere
/// this turn, exile that card instead.", held as a floating replacement on the
/// one card discarded (the engine mints a resolution-created replacement only
/// as a floating instance on a subject). The cleanup step does nothing else,
/// so nobody gets priority and the turn ends.
#[test]
fn a_this_turn_replacement_applies_to_the_cleanup_discard() {
    let mut state = game_with("Grizzly Bears", 1);
    // The replacement is created in turn 1's end step.
    drive_to_end_step(&mut state);
    let discarded = state.zones.hands[0][0];
    let card = state.objects.obj(discarded).card_id();
    state.shields.push(ReplacementInstance {
        id: InstanceId(0),
        replacement: Replacement::Instead {
            would: EventFilter::ZoneChange {
                what: Predicate::Any,
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            instead: Instruction::act(sylex_core::Action::move_to(
                Reference::Reg(sylex_core::RefId(2)),
                Zone::Exile,
            )),
        },
        subject: discarded,
        duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        one_shot: false,
        source: discarded,
    });
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (player, count) = drive_to_cleanup_discard(&mut state);
    assert_eq!((player, count), (PlayerId(0), 1));
    state
        .submit_decision(Decision::Discard(vec![discarded]))
        .unwrap();

    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::TurnBegan(
                _,
            )))) => break,
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(other) => {
                panic!("cleanup offers no decision when nothing waits: {other:?}")
            }
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
    assert!(
        state
            .zones
            .exile
            .iter()
            .any(|&o| state.objects.obj(o).card_id() == card),
        "the \"this turn\" replacement exiled the discarded card"
    );
    assert_eq!(
        state.zones.graveyards[0],
        [] as [ObjectId; 0],
        "nothing reached the graveyard"
    );
    assert!(
        state.shields.is_empty(),
        "the \"this turn\" replacement ended in cleanup ([CR#514.2])"
    );
}
