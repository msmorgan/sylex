//! Resolution against targets that became illegal after announcement
//! ([CR#608.2b,608.3b]), driven through the public API with real cards.
//!
//! Each test casts a spell for player 0, lets player 1 respond while the spell
//! is on the stack, and reads the result once the stack is empty. The
//! response that makes a target illegal is a real card, Ranger's Guile
//! (hexproof) or Keeper of Kookus (protection from red), spelled in semantic
//! RON and lowered through the production `Plugin::card_from_str` path, since
//! neither card is in the canon plugin.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use sylex_card::Card;
use sylex_core::Color;
use sylex_core::PhaseStep;
use sylex_core::Zone;
use sylex_engine::Action;
use sylex_engine::ChooseTargets;
use sylex_engine::DamageDealt;
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
use sylex_engine::TriggerFired;
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

/// Keeper of Kookus: "{R}: Keeper of Kookus gains protection from red until
/// end of turn."
const KEEPER_OF_KOOKUS: &str = r#"Normal(
    name: "Keeper of Kookus",
    mana_cost: [Red],
    types: [Creature],
    subtypes: [Goblin],
    abilities: [
        Activated(
            cost: [Mana([Red])],
            effect: Continuously(
                effect: Modify(This, GainAbility(Keyword(Protection(ColorIs(Red))))),
                duration: FixedUntil(EndOfTurn),
            ),
        ),
    ],
    power: 1,
    toughness: 1,
)"#;

/// Derived from Multani's Presence ("Whenever a spell you've cast is
/// countered, draw a card."): its trigger reads a spell's move from the stack
/// to a graveyard narrowed to the "Counter" cause verb [CR#701.6a].
const MULTANIS_PRESENCE: &str = r#"Normal(
    name: "Multani's Presence",
    mana_cost: [Green],
    types: [Enchantment],
    abilities: [
        Triggered(
            event: ZoneChange(
                what: ControlledBy(Ref(You)),
                from: Stack,
                to: Graveyard,
                cause: Cause(verb: Counter),
            ),
            effect: Draw(1),
        ),
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

fn count_named(state: &GameState, ids: &[ObjectId], name: &str) -> usize {
    ids.iter().filter(|&&o| is_card(state, o, name)).count()
}

/// Every event the trace applied, batch members included.
fn applied(trace: &[Progress]) -> Vec<&GameEvent> {
    trace
        .iter()
        .flat_map(|progress| match progress {
            Progress::Applied(Occurrence::Single(event)) => vec![event],
            Progress::Applied(Occurrence::Batch(events)) => events.iter().collect(),
            _ => Vec::new(),
        })
        .collect()
}

/// The damage events the trace applied to `target`.
fn damage_to(trace: &[Progress], target: ObjectId) -> Vec<DamageDealt> {
    applied(trace)
        .into_iter()
        .filter_map(|event| match event {
            GameEvent::DamageDealt(damage) if damage.target == target => Some(damage.clone()),
            _ => None,
        })
        .collect()
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

/// Player 0 casts `spell` with `targets`; at player 1's first priority with
/// that spell on the stack, player 1 takes `response` and gives it
/// `response_targets` when it targets. Runs until the stack is empty again
/// and returns the trace and the candidate sets player 0's target prompt
/// offered.
fn cast_against_response(
    state: &mut GameState,
    spell: ObjectId,
    targets: Vec<Vec<ObjectId>>,
    response: &Action,
    response_targets: Option<Vec<Vec<ObjectId>>>,
) -> (Vec<Progress>, Vec<Vec<ObjectId>>) {
    let mut picks: VecDeque<_> = std::iter::once(targets).chain(response_targets).collect();
    let mut offered = None;
    let mut responded = false;
    let mut answer = |state: &GameState, pending: &DecisionPointKind| match pending {
        DecisionPointKind::ChooseTargets(ChooseTargets { legal, .. }) => {
            offered.get_or_insert_with(|| legal.clone());
            Some(Decision::Targets(
                picks.pop_front().expect("a target set for every prompt"),
            ))
        }
        DecisionPointKind::Priority(Priority { player, .. })
            if *player == PlayerId(1) && !responded && !state.stack.is_empty() =>
        {
            responded = true;
            Some(Decision::Act(response.clone()))
        }
        _ => None,
    };
    let (_, stop) = step_to_stop(state);
    assert!(
        matches!(
            stop,
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(Priority { player, .. }))
                if player == PlayerId(0)
        ),
        "player 0 holds priority before casting, got {stop:?}"
    );
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("the spell is castable");
    let trace = drive(state, |s, _| s.stack.is_empty(), &mut answer);
    assert!(
        responded,
        "player 1 responded while the spell was on the stack"
    );
    assert!(picks.is_empty(), "every target prompt was answered");
    (trace, offered.expect("player 0's spell asked for targets"))
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

/// [CR#608.2b]: Pounce's opposing target gained hexproof, so it is illegal,
/// while player 0's own creature is still legal and Pounce resolves. Fight is
/// all or nothing: "if one or both creatures are illegal targets for a
/// resolving spell or ability that instructs them to fight, neither of them
/// fights or deals damage" [CR#701.14b]. Neither creature deals or is dealt
/// damage.
#[test]
fn pounce_whose_opposing_target_gained_hexproof_makes_no_fight() {
    let pounce = card("Pounce");
    let bears = card("Grizzly Bears");
    let forest = basic("Forest");
    let guile = fixture(RANGERS_GUILE);
    let mut state = game(
        deck(&[(&pounce, 2), (&bears, 2), (&forest, 6)]),
        deck(&[(&guile, 2), (&bears, 2), (&forest, 6)]),
    );
    to_main(&mut state);
    let mine = onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let spell = into_hand(&mut state, PlayerId(0), "Pounce");
    let response = into_hand(&mut state, PlayerId(1), "Ranger's Guile");
    float(
        &mut state,
        &[
            (PlayerId(0), Color::Green, 2),
            (PlayerId(1), Color::Green, 1),
        ],
    );

    let (trace, _) = cast_against_response(
        &mut state,
        spell,
        vec![vec![mine], vec![theirs]],
        &Action::CastSpell { object: response },
        Some(vec![vec![theirs]]),
    );

    assert!(
        damage_to(&trace, mine).is_empty() && damage_to(&trace, theirs).is_empty(),
        "no fight: neither creature deals or is dealt damage"
    );
    assert!(
        state.zones.battlefield.contains(&mine) && state.zones.battlefield.contains(&theirs),
        "both creatures are still on the battlefield"
    );
    assert_eq!(state.objects.obj(mine).total_damage(), 0);
    assert_eq!(state.objects.obj(theirs).total_damage(), 0);
    assert_eq!(
        count_named(&state, &state.zones.graveyards[0], "Pounce"),
        1,
        "Pounce resolved and went to its owner's graveyard"
    );
}

/// [CR#608.2b,115.7f]: Arc Lightning divides its 3 damage between two
/// creatures, 2 to the first and 1 to the second (the engine's even split
/// stands in for the announced division). The second gains hexproof before
/// Arc Lightning resolves, so it is an illegal target and is dealt nothing,
/// and the first is still dealt only its 2 (Arc Lightning ruling: "the
/// original division of damage still applies but no damage is dealt to the
/// illegal targets").
#[test]
fn arc_lightning_does_not_damage_a_target_that_gained_hexproof() {
    let arc = card("Arc Lightning");
    let bears = card("Grizzly Bears");
    let mountain = basic("Mountain");
    let forest = basic("Forest");
    let guile = fixture(RANGERS_GUILE);
    let mut state = game(
        deck(&[(&arc, 2), (&mountain, 8)]),
        deck(&[(&guile, 2), (&bears, 3), (&forest, 5)]),
    );
    to_main(&mut state);
    let hit = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let shielded = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let spell = into_hand(&mut state, PlayerId(0), "Arc Lightning");
    let response = into_hand(&mut state, PlayerId(1), "Ranger's Guile");
    float(
        &mut state,
        &[(PlayerId(0), Color::Red, 3), (PlayerId(1), Color::Green, 1)],
    );

    let (trace, _) = cast_against_response(
        &mut state,
        spell,
        vec![vec![hit, shielded]],
        &Action::CastSpell { object: response },
        Some(vec![vec![shielded]]),
    );

    assert!(
        damage_to(&trace, shielded).is_empty(),
        "the creature that gained hexproof is an illegal target and is dealt no damage"
    );
    assert_eq!(
        damage_to(&trace, hit)
            .iter()
            .map(|d| d.amount)
            .collect::<Vec<_>>(),
        [2],
        "the still-legal creature is dealt only its own share of the division"
    );
    assert!(state.zones.battlefield.contains(&shielded));
    assert_eq!(state.objects.obj(shielded).total_damage(), 0);
}

/// [CR#608.2b]: the same with protection from red. Keeper of Kookus gains
/// protection from red in response and is dealt no damage; the other target
/// is. Protection also prevents the damage [CR#702.16e], so this outcome
/// holds even where an illegal target is wrongly affected; the hexproof test
/// above is the one that tells the two apart.
#[test]
fn arc_lightning_does_not_damage_a_target_that_gained_protection_from_red() {
    let arc = card("Arc Lightning");
    let bears = card("Grizzly Bears");
    let mountain = basic("Mountain");
    let keeper = fixture(KEEPER_OF_KOOKUS);
    let mut state = game(
        deck(&[(&arc, 2), (&mountain, 8)]),
        deck(&[(&keeper, 2), (&bears, 2), (&mountain, 6)]),
    );
    to_main(&mut state);
    let hit = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let shielded = onto_battlefield(&mut state, PlayerId(1), "Keeper of Kookus");
    let spell = into_hand(&mut state, PlayerId(0), "Arc Lightning");
    float(
        &mut state,
        &[(PlayerId(0), Color::Red, 3), (PlayerId(1), Color::Red, 1)],
    );

    let (trace, _) = cast_against_response(
        &mut state,
        spell,
        vec![vec![hit, shielded]],
        &Action::ActivateAbility {
            object: shielded,
            ability: 0,
        },
        None,
    );

    assert!(
        damage_to(&trace, shielded).is_empty(),
        "Keeper of Kookus has protection from red and is dealt no damage"
    );
    assert_eq!(
        damage_to(&trace, hit)
            .iter()
            .map(|d| d.amount)
            .collect::<Vec<_>>(),
        [2],
        "the still-legal creature is dealt only its own share of the division"
    );
    assert!(state.zones.battlefield.contains(&shielded));
    assert_eq!(state.objects.obj(shielded).total_damage(), 0);
}

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

/// [CR#608.3b]: Pacifism's target gained hexproof before Pacifism resolved.
/// An Aura spell with an illegal target doesn't resolve: it goes to its
/// owner's graveyard without entering the battlefield.
#[test]
fn pacifism_with_an_illegal_target_goes_to_the_graveyard_without_entering() {
    let mut board = aura_board();
    let theirs = board.theirs;
    let state = &mut board.state;
    let spell = into_hand(state, PlayerId(0), "Pacifism");
    let response = into_hand(state, PlayerId(1), "Ranger's Guile");
    float(
        state,
        &[
            (PlayerId(0), Color::White, 2),
            (PlayerId(1), Color::Green, 1),
        ],
    );

    let (trace, offered) = cast_against_response(
        state,
        spell,
        vec![vec![theirs]],
        &Action::CastSpell { object: response },
        Some(vec![vec![theirs]]),
    );

    assert_eq!(
        count_named(state, &state.zones.battlefield, "Pacifism"),
        0,
        "Pacifism never entered the battlefield"
    );
    assert_eq!(
        count_named(state, &state.zones.graveyards[0], "Pacifism"),
        1,
        "Pacifism went to its owner's graveyard"
    );
    assert!(
        !applied(&trace).iter().any(|event| matches!(
            event,
            GameEvent::ZoneChange(change) if change.to == Zone::Battlefield
        )),
        "nothing entered the battlefield"
    );
    assert_creatures_on_the_battlefield(&board, &offered);
}

/// The control for the test above: an Aura spell targets [CR#303.4a], and
/// Pacifism whose target stays legal enters attached to it [CR#608.3c].
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

/// [CR#608.2b,701.6a]: Lightning Bolt's only target gained hexproof, so Bolt
/// doesn't resolve and goes to its owner's graveyard. Nothing countered it, so
/// player 0's "whenever a spell you control is countered" trigger doesn't
/// fire (Multani's Presence ruling: "A spell that doesn't resolve because its
/// targets are illegal isn't countered, so this ability won't trigger").
#[test]
fn a_spell_whose_only_target_became_illegal_is_not_countered() {
    let bolt = card("Lightning Bolt");
    let mountain = basic("Mountain");
    let presence = fixture(MULTANIS_PRESENCE);
    let bears = card("Grizzly Bears");
    let forest = basic("Forest");
    let guile = fixture(RANGERS_GUILE);
    let mut state = game(
        deck(&[(&bolt, 2), (&presence, 1), (&mountain, 7)]),
        deck(&[(&guile, 2), (&bears, 2), (&forest, 6)]),
    );
    to_main(&mut state);
    let watcher = onto_battlefield(&mut state, PlayerId(0), "Multani's Presence");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let spell = into_hand(&mut state, PlayerId(0), "Lightning Bolt");
    let response = into_hand(&mut state, PlayerId(1), "Ranger's Guile");
    float(
        &mut state,
        &[(PlayerId(0), Color::Red, 1), (PlayerId(1), Color::Green, 1)],
    );
    let hand = state.zones.hands[0].len();

    let (trace, _) = cast_against_response(
        &mut state,
        spell,
        vec![vec![theirs]],
        &Action::CastSpell { object: response },
        Some(vec![vec![theirs]]),
    );

    assert_eq!(
        count_named(&state, &state.zones.graveyards[0], "Lightning Bolt"),
        1,
        "the Bolt went to its owner's graveyard"
    );
    assert!(
        damage_to(&trace, theirs).is_empty(),
        "the Bolt dealt no damage"
    );
    let watcher_source = state.objects.obj(watcher).source;
    assert!(
        !applied(&trace).iter().any(|event| matches!(
            event,
            GameEvent::TriggerFired(TriggerFired { source, .. }) if *source == watcher_source
        )),
        "the countered-spell trigger did not fire"
    );
    assert_eq!(
        state.zones.hands[0].len(),
        hand - 1,
        "player 0 drew nothing (the hand lost only the Bolt)"
    );
}

/// The control for the test above: Mana Leak counters the Bolt (player 0
/// can't pay {3}), and the same trigger fires and draws a card [CR#701.6a].
#[test]
fn a_countered_spell_fires_the_countered_spell_trigger() {
    let bolt = card("Lightning Bolt");
    let mountain = basic("Mountain");
    let presence = fixture(MULTANIS_PRESENCE);
    let leak = card("Mana Leak");
    let island = basic("Island");
    let bears = card("Grizzly Bears");
    let mut state = game(
        deck(&[(&bolt, 2), (&presence, 1), (&mountain, 7)]),
        deck(&[(&leak, 2), (&bears, 2), (&island, 6)]),
    );
    to_main(&mut state);
    let watcher = onto_battlefield(&mut state, PlayerId(0), "Multani's Presence");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let spell = into_hand(&mut state, PlayerId(0), "Lightning Bolt");
    let response = into_hand(&mut state, PlayerId(1), "Mana Leak");
    float(
        &mut state,
        &[(PlayerId(0), Color::Red, 1), (PlayerId(1), Color::Blue, 2)],
    );
    let hand = state.zones.hands[0].len();

    let (trace, _) = cast_against_response(
        &mut state,
        spell,
        vec![vec![theirs]],
        &Action::CastSpell { object: response },
        Some(vec![vec![spell]]),
    );

    assert!(
        damage_to(&trace, theirs).is_empty(),
        "the Bolt was countered"
    );
    let watcher_source = state.objects.obj(watcher).source;
    assert!(
        applied(&trace).iter().any(|event| matches!(
            event,
            GameEvent::TriggerFired(TriggerFired { source, .. }) if *source == watcher_source
        )),
        "the countered-spell trigger fired"
    );
    assert_eq!(
        state.zones.hands[0].len(),
        hand,
        "player 0 lost the Bolt from hand and drew one card"
    );
}
