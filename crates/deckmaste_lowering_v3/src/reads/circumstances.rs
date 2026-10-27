//! Timing, duration, quantity and location stepping.
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::Duration;
use deckmaste_semantics_v3::ast::GameEvent;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::Location;
use deckmaste_semantics_v3::ast::LookbackSpan;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Ordinal;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Quantity;
use deckmaste_semantics_v3::ast::Timing;
use deckmaste_semantics_v3::ast::TurnRef;

use super::DRS;
use super::Determiner;
use super::GramRole;
use super::StepOps;
use super::StepResult;
use crate::reads::KindResolutionExt;
use crate::reads::NounPhraseFactsExt;
use crate::reads::NounPhrasePhraseFactsExt;
use crate::refusal::Refusal;

pub(super) fn refuse(condition: bool, refusal: Refusal) -> Vec<Refusal> {
    if condition { Vec::new() } else { vec![refusal] }
}
pub(super) fn kind_check(expected: Kind, actual: Option<Kind>) -> Vec<Refusal> {
    actual.map_or_else(Vec::new, |found| {
        refuse(
            expected == found,
            Refusal::KindMismatch {
                expected: Box::new(expected),
                found: Box::new(found),
            },
        )
    })
}

pub fn step_quantity_with(ops: &impl StepOps, drs: DRS, value: &Quantity) -> StepResult {
    if ops.fuel() == 0 {
        return (drs, vec![Refusal::RecursionLimit]);
    }
    match value {
        Quantity::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.quantity(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        Quantity::Range {
            low: Some(low),
            high: Some(high),
        } => (drs, refuse(low <= high, Refusal::WellFormedQ)),
        Quantity::Range { .. } | Quantity::Worth { .. } | Quantity::TheLast => (drs, Vec::new()),
        Quantity::UpToOf { amount }
        | Quantity::ExactlyOf { amount }
        | Quantity::MoreThanOf { amount } => ops.amount(drs, amount),
        Quantity::Additional { quantity } => ops.quantity(drs, quantity),
    }
}

pub fn step_turn_ref_with(ops: &impl StepOps, drs: DRS, value: &TurnRef) -> StepResult {
    match value {
        TurnRef::Next {
            player: Some(owner),
        }
        | TurnRef::Each {
            player: Some(owner),
        }
        | TurnRef::Previous {
            player: Some(owner),
        }
        | TurnRef::MostRecent { player: owner } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, owner);
            errors.extend(kind_check(Kind::Player, owner.kind()));
            (out, errors)
        }
        TurnRef::That => {
            let found = drs
                .visible_referents()
                .filter(|r| Kind::TurnRef.lte(&r.payload.kind()) && r.plur == Plurality::One)
                .count() as u64;
            (drs, refuse(found == 1, Refusal::TurnAntecedent { found }))
        }
        TurnRef::Current
        | TurnRef::Previous { player: None }
        | TurnRef::Later
        | TurnRef::Next { player: None }
        | TurnRef::Each { player: None } => (drs, Vec::new()),
    }
}

fn duration_possessor_ok(noun: Option<&NounPhrase>) -> bool {
    noun.is_none_or(|noun| {
        noun.plurality() == Plurality::One
            && noun
                .determiner()
                .is_none_or(|det| matches!(det, Determiner::The | Determiner::Target))
    })
}
fn boundary_possessor_ok(event: Option<&GameEvent>) -> bool {
    match event {
        Some(GameEvent::BeginningOf { whose, .. } | GameEvent::EndOf { whose, .. }) => {
            duration_possessor_ok(whose.as_deref())
        }
        _ => true,
    }
}
pub fn step_duration_with(ops: &impl StepOps, mut drs: DRS, value: &Duration) -> StepResult {
    match value {
        Duration::Between { start, end_ } => {
            let (out, mut errors) = start
                .as_ref()
                .map_or((drs.clone(), Vec::new()), |start| ops.event(drs, start));
            let (out, end_errors) = end_
                .as_ref()
                .map_or((out.clone(), Vec::new()), |end_| ops.event(out, end_));
            errors.extend(end_errors);
            errors.extend(refuse(
                boundary_possessor_ok(start.as_deref()) && boundary_possessor_ok(end_.as_deref()),
                Refusal::DurationPossessor,
            ));
            (out, errors)
        }
        Duration::ForAsLongAs { condition } => {
            drs.next_clause();
            ops.condition(drs, condition)
        }
        Duration::ModifiesPrecedingAction => (drs, Vec::new()),
    }
}
pub fn step_timing_with(ops: &impl StepOps, drs: DRS, value: &Timing) -> StepResult {
    if ops.fuel() == 0 {
        return (drs, vec![Refusal::RecursionLimit]);
    }
    match value {
        Timing::InPart { whose, which, .. } => {
            let (out, mut errors) = if let Some(owner) = whose {
                let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, owner);
                errors.extend(kind_check(Kind::Player, owner.kind()));
                (out, errors)
            } else {
                (drs, Vec::new())
            };
            let (out, more) = step_turn_ref_with(ops, out, which);
            errors.extend(more);
            (out, errors)
        }
        Timing::AnyTiming => (drs, Vec::new()),
        Timing::RelativeToEvent { event, .. } | Timing::WhileDoing { event } => {
            ops.event(drs, event)
        }
        Timing::OnlyIf { condition } => ops.condition(drs, condition),
        Timing::Not { timing } => {
            let (_, errors) = ops.timing(drs.clone(), timing);
            (drs, errors)
        }
        Timing::And { conjuncts } => {
            conjuncts
                .iter()
                .fold((drs, Vec::new()), |(drs, mut errors), timing| {
                    let (out, more) = ops.timing(drs, timing);
                    errors.extend(more);
                    (out, errors)
                })
        }
        Timing::Or { disjuncts } => {
            let mut branches = Vec::new();
            let mut errors = Vec::new();
            for timing in disjuncts {
                let (out, more) = ops.timing(drs.clone(), timing);
                branches.push(out);
                errors.extend(more);
            }
            (drs.common_header_context(&branches), errors)
        }
    }
}
pub fn step_lookback_span_with(ops: &impl StepOps, drs: DRS, value: &LookbackSpan) -> StepResult {
    match value {
        LookbackSpan::Duration { d, timing } => {
            let (out, mut errors) = step_duration_with(ops, drs, d);
            let (out, more) = timing.as_ref().map_or((out.clone(), Vec::new()), |timing| {
                step_timing_with(ops, out, timing)
            });
            errors.extend(more);
            (out, errors)
        }
        LookbackSpan::ThisWay | LookbackSpan::Triggering | LookbackSpan::ThisObject => {
            (drs, Vec::new())
        }
    }
}
pub fn step_location_with(ops: &impl StepOps, drs: DRS, value: &Location) -> StepResult {
    if ops.fuel() == 0 {
        return (drs, vec![Refusal::RecursionLimit]);
    }
    match value {
        Location::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.location(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        Location::Zone { whose, .. } => {
            if let Some(owner) = whose {
                let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, owner);
                if !ids.is_empty() {
                    errors.extend(kind_check(
                        Kind::Player,
                        Some(owner.selected_kind(&out, &ids)),
                    ));
                }
                (out, errors)
            } else {
                (drs, Vec::new())
            }
        }
        Location::FromEnd {
            ordinal: Ordinal::Nth { n },
            loc,
            ..
        } => {
            let (out, mut errors) = ops.location(drs, loc);
            errors.extend(refuse(*n != 0, Refusal::OrdinalNonZero));
            (out, errors)
        }
        Location::ExiledWith { source } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, source);
            errors.extend(refuse(source.self_defined_ok(), Refusal::LinkSource));
            (out, errors)
        }
        Location::Pile { pile } => {
            let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, pile);
            errors.extend(kind_check(
                Kind::Pile,
                ids.first()
                    .and_then(|id| out.find_by_id(*id))
                    .map(|r| r.payload.kind()),
            ));
            errors.extend(refuse(pile.pile_mention(), Refusal::PileMention));
            (out, errors)
        }
        Location::AnyLocation => (drs, Vec::new()),
        Location::Not { loc } => {
            let (_, errors) = ops.location(drs.clone(), loc);
            (drs, errors)
        }
        Location::Coord {
            coordinator: Coordinator::Or,
            locs,
        } => {
            let mut branches = Vec::new();
            let mut errors = Vec::new();
            for loc in locs {
                let (out, more) = ops.location(drs.clone(), loc);
                branches.push(out);
                errors.extend(more);
            }
            errors.extend(refuse(locs.len() >= 2, Refusal::AtLeastTwoZones));
            (drs.common_header_context(&branches), errors)
        }
        Location::Coord { locs, .. } => {
            let (out, mut errors) =
                locs.iter()
                    .fold((drs, Vec::new()), |(drs, mut errors), loc| {
                        let (out, more) = ops.location(drs, loc);
                        errors.extend(more);
                        (out, errors)
                    });
            errors.extend(refuse(locs.len() >= 2, Refusal::AtLeastTwoZones));
            (out, errors)
        }
    }
}
