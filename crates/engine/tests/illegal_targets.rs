//! Resolution against announced targets, driven through the public API with
//! real cards.

use std::path::Path;
use std::sync::Arc;

use sylex_card::Card;
use sylex_core::Color;
use sylex_core::PhaseStep;
use sylex_core::Zone;
use sylex_engine::Action;
use sylex_engine::ChooseTargets;
use sylex_engine::Decision;
use sylex_engine::DecisionPointKind;
use sylex_engine::GameConfig;
use sylex_engine::GameEvent;
use sylex_engine::GameState;
use sylex_engine::ManaProvenance;
use sylex_engine::ObjectId;
use sylex_engine::Occurrence;
use sylex_engine::PlayerConfig;
use sylex_engine::PlayerId;
use sylex_engine::Priority;
use sylex_engine::Progress;
use sylex_engine::StartingPlayer;
use sylex_engine::StepOutcome;
use sylex_engine::WorkItem;
use sylex_engine::ZoneChange;
use sylex_plugin_v1::plugin::Plugin;

// --- plugins and cards -------------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../v1/plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../v1/plugins/canon"),
    )
    .unwrap()
}

fn card(name: &str) -> Arc<Card> {
    Arc::new(canon().card(name).unwrap().core)
}

fn basic(name: &str) -> Arc<Card> {
    Arc::new(builtin().card(name).unwrap().core)
}

/// A hand-spelled semantic card, lowered through the production boundary.
fn fixture(source: &str) -> Arc<Card> {
    Arc::new(
        canon()
            .card_from_str(source)
            .unwrap_or_else(|error| panic!("fixture did not load: {error}\n{source}"))
            .core,
    )
}

/// Ranger's Guile: "Target creature you control gets +1/+1 and gains hexproof
/// until end of turn."
const RANGERS_GUILE: &str = r#"Normal(
    name: "Ranger's Guile",
    mana_cost: [Green],
    types: [Instant],
    abilities: [
        Spell(effect: Targeted(
            targets: [TargetOne(And([Creature, ControlledBy(Ref(You))]))],
            effect: Continuously(
                effect: Modify(Target(0), Several([
                    Power(Up(1)),
                    Toughness(Up(1)),
                    GainAbility(Keyword(Hexproof())),
                ])),
                duration: FixedUntil(EndOfTurn),
            ),
        )),
    ],
)"#;

/// A two-player game with the builtin rules as data (lethal damage, the Aura
/// graveyard rule) and canon's type registries.
fn game(p0: Vec<Arc<Card>>, p1: Vec<Arc<Card>>) -> GameState {
    let builtin = builtin();
    let canon = canon();
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: builtin.sba_rules.clone(),
        conferral_rules: builtin.conferral_rules.clone(),
        damage_result_rules: builtin.damage_result_rules.clone(),
        counter_decls: canon.counters.clone(),
        subtypes: canon.subtypes.clone(),
        types: canon.types.clone(),
    })
}

/// `copies` of each card, concatenated into one deck.
fn deck(cards: &[(&Arc<Card>, usize)]) -> Vec<Arc<Card>> {
    cards
        .iter()
        .flat_map(|(card, copies)| std::iter::repeat_n(Arc::clone(card), *copies))
        .collect()
}

// --- objects -----------------------------------------------------------------

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
    }
}

fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == name)
}

/// Moves the first `name` card `player` owns (library, then hand) into their
/// hand.
fn into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        state.zones.libraries[i].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Hand);
        state.zones.hands[i].push(obj);
        return obj;
    }
    *state.zones.hands[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} owned by player {}", player.0))
}

/// Moves the first `name` card `player` owns straight onto the battlefield.
fn onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = into_hand(state, player, name);
    state.zones.hands[player.index()].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Moves the first `name` card `player` owns (library, then hand) into their
/// graveyard.
fn into_graveyard(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = into_hand(state, player, name);
    state.zones.hands[player.index()].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Graveyard);
    state.zones.graveyards[player.index()].push(obj);
    obj
}

fn sorted(mut ids: Vec<ObjectId>) -> Vec<ObjectId> {
    ids.sort_unstable();
    ids
}

// --- driving -----------------------------------------------------------------

fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

/// Answers every decision with `answer` first, then the routine answer
/// (pass priority, auto-pay, keep trigger order), until `stop` accepts a
/// priority window. A prompt neither handles panics.
fn drive<S, A>(state: &mut GameState, stop: S, answer: &mut A) -> Vec<Progress>
where
    S: Fn(&GameState, PlayerId) -> bool,
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    let mut trace = Vec::new();
    for _ in 0..2000 {
        let (more, outcome) = step_to_stop(state);
        trace.extend(more);
        let pending = match outcome {
            StepOutcome::NeedsDecision(pending) => pending,
            other => panic!("the game ended before the test's stop: {other:?}"),
        };
        if let DecisionPointKind::Priority(Priority { player, .. }) = &pending
            && stop(state, *player)
        {
            return trace;
        }
        let decision = answer(state, &pending)
            .or_else(|| match &pending {
                DecisionPointKind::Priority(_) => Some(Decision::Act(Action::Pass)),
                DecisionPointKind::PayMana(_) => Some(Decision::Pay(state.auto_pay_pending())),
                DecisionPointKind::Payment(_) => state.auto_payment_pending(),
                DecisionPointKind::OrderTriggers(prompt) => {
                    Some(Decision::Order((0..prompt.triggers.len()).collect()))
                }
                DecisionPointKind::DeclareAttackers(_) => Some(Decision::Attackers(Vec::new())),
                DecisionPointKind::DeclareBlockers(_) => Some(Decision::Blocks(Vec::new())),
                _ => None,
            })
            .unwrap_or_else(|| panic!("the test has no answer for {pending:?}"));
        state
            .submit_decision(decision)
            .unwrap_or_else(|error| panic!("{error} answering {pending:?}"));
    }
    panic!("the test never reached its stop");
}

/// Floats mana into each listed player's pool (`n` of `color`) and reopens
/// the priority window in flight so the pools show in the legal list.
fn float(state: &mut GameState, pools: &[(PlayerId, Color, u32)]) {
    for &(player, color, n) in pools {
        state
            .player_mut(player)
            .mana_pool
            .add(color.into(), n, ManaProvenance::default());
    }
    assert!(
        matches!(state.pending, Some(DecisionPointKind::Priority(_))),
        "float expects a priority window in flight"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

/// Runs to player 0's precombat main phase.
fn to_main(state: &mut GameState) {
    drive(
        state,
        |s, p| p == PlayerId(0) && s.turn.current == PhaseStep::PrecombatMain,
        &mut |_, _| None,
    );
}

// --- tests -------------------------------------------------------------------

/// A board for the Aura tests: a creature on each side of the battlefield,
/// plus Grizzly Bears cards in player 1's hand and player 0's graveyard that
/// no "enchant creature" Aura may target or enchant [CR#109.2].
struct AuraBoard {
    state: GameState,
    mine: ObjectId,
    theirs: ObjectId,
    in_hand: ObjectId,
    in_graveyard: ObjectId,
}

fn aura_board() -> AuraBoard {
    let pacifism = card("Pacifism");
    let plains = basic("Plains");
    let bears = card("Grizzly Bears");
    let forest = basic("Forest");
    let guile = fixture(RANGERS_GUILE);
    let mut state = game(
        deck(&[(&pacifism, 2), (&bears, 3), (&plains, 5)]),
        deck(&[(&guile, 2), (&bears, 3), (&forest, 5)]),
    );
    to_main(&mut state);
    let in_graveyard = into_graveyard(&mut state, PlayerId(0), "Grizzly Bears");
    let mine = onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let in_hand = into_hand(&mut state, PlayerId(1), "Grizzly Bears");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    onto_battlefield(&mut state, PlayerId(0), "Plains");
    AuraBoard {
        state,
        mine,
        theirs,
        in_hand,
        in_graveyard,
    }
}

/// [CR#303.4a,109.2]: Pacifism's target menu is exactly the creatures on the
/// battlefield, never a creature card in a hand or a graveyard.
fn assert_creatures_on_the_battlefield(board: &AuraBoard, offered: &[Vec<ObjectId>]) {
    assert_eq!(offered.len(), 1, "Pacifism announces one target slot");
    assert_eq!(
        sorted(offered[0].clone()),
        sorted(vec![board.mine, board.theirs]),
        "Pacifism may target exactly the creatures on the battlefield, not the cards in a hand \
         ({:?}) or a graveyard ({:?})",
        board.in_hand,
        board.in_graveyard,
    );
}

/// An Aura spell targets [CR#303.4a]: Pacifism announces its enchant target,
/// and on resolution enters attached to it [CR#608.3c].
#[test]
fn pacifism_with_a_legal_target_enters_attached_to_it() {
    let mut board = aura_board();
    let theirs = board.theirs;
    let state = &mut board.state;
    let spell = into_hand(state, PlayerId(0), "Pacifism");
    float(state, &[(PlayerId(0), Color::White, 2)]);
    let (_, stop) = step_to_stop(state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Priority(_))
    ));
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("Pacifism is castable");
    let mut offered = Vec::new();
    drive(
        state,
        |s, _| s.stack.is_empty(),
        &mut |_, pending: &DecisionPointKind| match pending {
            DecisionPointKind::ChooseTargets(ChooseTargets { legal, .. }) => {
                offered.clone_from(legal);
                Some(Decision::Targets(vec![vec![theirs]]))
            }
            _ => None,
        },
    );

    let aura = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&o| is_card(state, o, "Pacifism"))
        .expect("Pacifism entered the battlefield");
    assert_eq!(state.objects.obj(aura).attached_to, Some(theirs));
    assert_creatures_on_the_battlefield(&board, &offered);
}

/// [CR#303.4f,109.2]: an Aura put onto the battlefield other than by
/// resolving, by an effect that names no host, enters attached to a creature
/// on the battlefield its controller chooses as it enters, never to a
/// creature card in a hand or a graveyard. Player 0's own Grizzly Bears cards
/// in their hand and graveyard come first in the engine's object order, so a
/// host choice that admitted them would pick one.
#[test]
fn pacifism_entering_without_being_cast_chooses_a_creature_on_the_battlefield() {
    let pacifism = card("Pacifism");
    let plains = basic("Plains");
    let bears = card("Grizzly Bears");
    let forest = basic("Forest");
    let mut state = game(
        deck(&[(&bears, 3), (&pacifism, 2), (&plains, 5)]),
        deck(&[(&bears, 3), (&forest, 7)]),
    );
    to_main(&mut state);
    let in_hand = into_hand(&mut state, PlayerId(0), "Grizzly Bears");
    let in_graveyard = into_graveyard(&mut state, PlayerId(0), "Grizzly Bears");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let aura = into_hand(&mut state, PlayerId(0), "Pacifism");
    assert!(
        in_hand < theirs && in_graveyard < theirs,
        "the cards outside the battlefield come first in object order"
    );
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::Priority(_))
    ));
    state.pending = None;
    state
        .agenda
        .push_front(WorkItem::Emit(Occurrence::single(GameEvent::ZoneChange(
            ZoneChange {
                snapshot: None,
                object: aura,
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))));
    state.agenda.push_back(WorkItem::OpenPriority);
    drive(&mut state, |_, _| true, &mut |_, _| None);

    let aura = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&o| is_card(&state, o, "Pacifism"))
        .expect("Pacifism entered the battlefield");
    assert_eq!(
        state.objects.obj(aura).attached_to,
        Some(theirs),
        "Pacifism chose the only creature on the battlefield"
    );
}
