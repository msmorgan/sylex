//! A replacement's body: the modified event that occurs instead of the
//! replaced one ([CR#614.6]), and how it joins the occurrence it replaces in.
//!
//! A replaced event never happens; a modified event occurs instead
//! [CR#614.6]. When the replaced event is one member of a simultaneous event,
//! the modified event takes its place in that event: the members of one
//! destroy-all that Rest in Peace exiles are exiled together, and the
//! creatures one destroy-all regenerates become tapped together, so a "one or
//! more" watcher sees one event [CR#603.2c]. A body's parts join that event
//! until one can't be processed with it, the way one action on several
//! objects is processed simultaneously unless it can't be [CR#608.2f]. A part
//! can't be processed with the event when:
//!
//! - it is not a verb (a choice, a condition, a `Let`, a nested sequence), or
//!   its verb lowers to more than events or yields a magnitude: it needs the
//!   event to have happened;
//! - it draws: the draws happen after any unreplaced part of the event, one at
//!   a time [CR#121.7];
//! - an earlier part did more than heal or tap: a part reads the game as it is
//!   before the event, and one that makes an object, moves one (which makes it
//!   a new object [CR#400.7]), or changes a characteristic, a controller or a
//!   counter may change what the later part acts on;
//! - a "for this event" rider follows it: the rider is armed for the part
//!   before it (`run_effect`'s `Sequentially`), so that part waits with it.
//!
//! The parts before the first that can't join the occurrence; that part and
//! every later one wait. Where the waiting parts go is the engine's choice:
//! the CR orders only draws [CR#121.7]. They run after the event, in order,
//! one member at a time: each member's waiting parts are their own work, and
//! each emits its own occurrence. So the later parts of the bodies that
//! replace several members of one event commit as one occurrence per member
//! (`engine-replacement-body-tails-split-the-event`).

use std::sync::Arc;

use sylex_core::Action;
use sylex_core::Instruction;

use super::ReplacementKey;
use crate::agenda::WorkItem;
use crate::event::Act;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::ExecutionFrame;
use crate::state::GameState;
use crate::state::OccurrencePass;
use crate::trigger::EventPatient;

/// Join a replacement's `instead`/`also` body to the occurrence `pass`.
///
/// The body's frame is anchored on `source`, with the replaced intent's
/// affected recipient bound for the body to read while `This` stays the
/// source ability. A card-backed recipient binds as `EventObject` (a body
/// reads `Ref(EventObject)` — regeneration heals it, wither/infect put -1/-1
/// counters on it, [CR#702.80a,702.90c]); a PLAYER recipient binds as the
/// kind-poly `EventPatient` — infect gives that player poison counters
/// ([CR#702.90b]). `EventActor` names the responsible AGENT ([CR#119.9]'s
/// distinction), and a replaced event's affected recipient is never that.
///
/// The parts that happen with the rest of the event (see the module docs)
/// join `pass.joining` with `applied`, the [CR#614.5] lineage already spent
/// against the event this body replaces (including the very key that just
/// fired), so none of those replacements applies to them again; their
/// keyword-action finalizers and the parts that must wait go to
/// `pass.after`. The frame carries `applied` as well, so a part that waits
/// starts its own pass pre-excluding it ([`super::inherited_seed`]).
#[expect(
    clippy::too_many_arguments,
    reason = "one parameter per piece of the replaced event the body's frame binds"
)]
pub(crate) fn join_body(
    state: &mut GameState,
    body: Instruction,
    source: ObjectId,
    that: Option<ObjectId>,
    applied: &std::collections::HashSet<ReplacementKey>,
    params: Arc<[sylex_core::Param]>,
    event_amount: Option<sylex_core::Uint>,
    pass: &mut OccurrencePass,
) {
    let controller = state.controller_of(source);
    // [CR#608.2,608.2k]: bind the affected recipient — the event PATIENT
    // ([CR#120.3]) — so the body can read it while `This` falls back to
    // `source` (`bindings.this` is `None`, the agent never moving off the
    // ability). A frameless body (`that == None`) leaves bindings unset. The
    // recipient is the provenance-explicit `EventPatient`; a card/token
    // recipient ALSO mirrors into `that_object` so `EventObject` bodies keep
    // reading it (regeneration/wither/infect-on-a-creature).
    let (event_object, event_patient) =
        that.map_or((None, None), |id| match state.objects.obj(id).source {
            ObjectSource::Player(p) => (None, Some(EventPatient::Player(p))),
            ObjectSource::Card(_) => {
                let snapshot = LkiSnapshot::capture(state, id);
                (Some(snapshot.clone()), Some(EventPatient::Object(snapshot)))
            }
        });
    let mut frame = state.frame(source, controller);
    state.frame_set_action_context(&mut frame, applied.clone(), false);
    state.frame_set_event_bindings(&mut frame, event_object, None, event_patient);
    // [CR#107.3]: the replaced intent's own magnitude reaches the body through
    // the region's `EventAmount` parameter.
    state.frame_set_event_extras(&mut frame, event_amount, Vec::new(), None);
    frame.activation = state.enter_region(&sylex_core::Region::new(params, ()), &frame);

    let parts: Vec<Instruction> = match &body {
        Instruction::Sequentially(parts) => parts.to_vec(),
        part => vec![part.clone()],
    };
    let mut joining = Vec::new();
    let mut changed = false;
    for part in &parts {
        if changed {
            break;
        }
        let Some((events, finalizers)) = simultaneous_part(state, part, &frame) else {
            break;
        };
        changed = !events.iter().all(heals_or_taps);
        joining.push((events, finalizers));
    }
    // A "for this event" rider is armed for the part before it, which waits
    // with it.
    if parts
        .get(joining.len())
        .is_some_and(|part| crate::resolve::for_this_event_rider(part).is_some())
    {
        joining.pop();
    }
    let joined = joining.len();
    for (events, finalizers) in joining {
        pass.joining
            .extend(events.into_iter().map(|event| (event, applied.clone())));
        pass.after.extend(finalizers);
    }
    if joined < parts.len() {
        let effect = if joined == 0 {
            body
        } else {
            Instruction::Sequentially(parts[joined..].into())
        };
        pass.after.push(WorkItem::RunEffect {
            effect: Arc::new(effect),
            frame,
        });
    }
}

/// The events `part` produces and the finalizers that follow their commit,
/// when it can happen at the same time as the rest of its event: a verb that
/// lowers to events alone, none of them a draw ([CR#121.7]), and whose
/// product, if it names one, is known before it runs. The product is written
/// as `run_effect` writes it — at the object's pre-move identity, chased to
/// its new one on read [CR#400.7]. `None` when it must wait for the event to
/// happen.
fn simultaneous_part(
    state: &GameState,
    part: &Instruction,
    frame: &ExecutionFrame,
) -> Option<(Vec<GameEvent>, Vec<WorkItem>)> {
    let Instruction::Act { dest, action } = part else {
        return None;
    };
    // Creating a replacement mutates the shield list at run time, not through
    // an event; a runtime magnitude is counted off the events once they
    // happened.
    if matches!(action, Action::CreateReplacement { .. })
        || (dest.is_some() && action.produces_runtime_magnitude())
    {
        return None;
    }
    let mut events = Vec::new();
    let mut finalizers = Vec::new();
    for item in state.action_items(action, frame) {
        match item {
            WorkItem::Emit(Occurrence::Single(event)) => events.push(event),
            WorkItem::Emit(Occurrence::Batch(batch)) => events.extend(batch),
            finalizer @ WorkItem::FinalizeAct { .. } => finalizers.push(finalizer),
            // Work that must run before what follows it can be known: a
            // decision, a coin flip or die roll, a reposition, a nested
            // effect, a cast or activation, and gaining control, which creates
            // a timestamped continuous effect when it runs and only then knows
            // whether the controller changed.
            WorkItem::BeginStep(_)
            | WorkItem::CheckSbas
            | WorkItem::PlaceTriggers
            | WorkItem::CheckHandSize
            | WorkItem::EndTurnEffects
            | WorkItem::CleanupCheck
            | WorkItem::CleanupPriority { .. }
            | WorkItem::DeclareAttackers
            | WorkItem::DeclareBlockers
            | WorkItem::AssignCombatDamage
            | WorkItem::EndOfCombat
            | WorkItem::OpenPriority
            | WorkItem::BeginCast(_)
            | WorkItem::BeginCastFromResolution { .. }
            | WorkItem::BeginActivate { .. }
            | WorkItem::AnnounceModes
            | WorkItem::AnnounceOptionalCosts { .. }
            | WorkItem::AnnounceX
            | WorkItem::AnnounceTargets
            | WorkItem::ChooseCostOptions
            | WorkItem::OpenPayment
            | WorkItem::FinishPaymentFulfillment(_)
            | WorkItem::BeginManaAction(_)
            | WorkItem::FinishManaAction(_)
            | WorkItem::ResolveTriggeredMana { .. }
            | WorkItem::FinishTriggeredMana { .. }
            | WorkItem::CompleteTriggeredMana(_)
            | WorkItem::FlipCoins { .. }
            | WorkItem::RollDice { .. }
            | WorkItem::ChooseManaColor { .. }
            | WorkItem::ChooseManaMode { .. }
            | WorkItem::TollMana { .. }
            | WorkItem::Retarget { .. }
            | WorkItem::ChooseNoteNumber { .. }
            | WorkItem::ChooseNoteCardName { .. }
            | WorkItem::Resolve(_)
            | WorkItem::RunEffect { .. }
            | WorkItem::InstallRiders { .. }
            | WorkItem::RepositionLibrary { .. }
            | WorkItem::ArrangePiles
            | WorkItem::ArrangeGroupLanding { .. }
            | WorkItem::WriteMagnitude { .. }
            | WorkItem::GainControl { .. } => return None,
            WorkItem::EmitSbaBatch(_) | WorkItem::CommitReplaced { .. } => unreachable!(
                "a verb lowered to a state-based or replaced commit, which only the \
                 state-based check and the replacement pass schedule"
            ),
        }
    }
    if events
        .iter()
        .any(|event| matches!(event, GameEvent::Act(Act { verb, .. }) if verb.0.as_str() == "Draw"))
    {
        return None;
    }
    if let Some(dest) = dest {
        state.write_act_product(*dest, action, frame);
    }
    Some((events, finalizers))
}

/// Whether `event` only removes damage from a permanent or taps or untaps it
/// — regeneration's heal and tap [CR#701.19a] — so a later part of the same
/// body finds the same objects, in the same zones, with the same
/// characteristics and controllers.
fn heals_or_taps(event: &GameEvent) -> bool {
    matches!(
        event,
        GameEvent::DamageRemoved(_) | GameEvent::Tapped(_) | GameEvent::Untapped(..)
    )
}
