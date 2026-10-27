//! Conditional clauses and the referents their headers publish.
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::Condition;
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::DesignationLabel;
use deckmaste_semantics_v3::ast::DesignationScope;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::LookbackSpan;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::SlotCarrier;

use super::DRS;
use super::Determiner;
use super::GramRole;
use super::Payload;
use super::Referent;
use super::StepOps;
use super::StepResult;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::reads::NounPhraseFactsExt;
use crate::reads::OutcomeSortWordsExt;
use crate::reads::PredicateFactsExt;
use crate::refusal::Refusal;
/// Discourse inspection of `Predicate`.
pub trait PredicateConditionsExt: Sized {
    #[must_use]
    fn says(&self) -> bool;
    #[doc(hidden)]
    fn says_at(&self, fuel: u64) -> bool;
}

impl PredicateConditionsExt for Predicate {
    fn says(&self) -> bool {
        self.says_at(64)
    }
    fn says_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body } | Self::Not { predicate: body }) => {
                body.says_at(fuel - 1)
            }
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().any(|p| p.says_at(fuel - 1)),
            _ => true,
        }
    }
}
/// Discourse inspection of `Amount`.
pub trait AmountConditionsExt: Sized {
    #[must_use]
    fn read(&self) -> bool;
}

impl AmountConditionsExt for Amount {
    fn read(&self) -> bool {
        let mut amount = self;
        for _ in 0..64 {
            match amount {
                Self::InCaller { body } => amount = body,
                Self::Lit { .. }
                | Self::Arith { .. }
                | Self::ThatMuch
                | Self::ChosenNumber { .. }
                | Self::GroupSize
                | Self::Half { .. } => return false,
                Self::TheOutcome { sort } => return sort.comparable(),
                _ => return true,
            }
        }
        true
    }
}
/// Discourse inspection of `Condition`.
pub trait ConditionExt: Sized {
    #[must_use]
    fn is_and(&self) -> bool;
    #[must_use]
    fn is_or(&self) -> bool;
    #[doc(hidden)]
    fn scope_body(&self) -> Option<&Self>;
}

impl ConditionExt for Condition {
    fn is_and(&self) -> bool {
        matches!(self.scope_body(), Some(Self::And { .. }))
    }
    fn is_or(&self) -> bool {
        matches!(self.scope_body(), Some(Self::Or { .. }))
    }
    fn scope_body(&self) -> Option<&Self> {
        let mut condition = self;
        for _ in 0..64 {
            match condition {
                Self::InCaller { body } => condition = body,
                _ => return Some(condition),
            }
        }
        None
    }
}
pub fn step_condition_with(ops: &impl StepOps, drs: DRS, condition: &Condition) -> StepResult {
    let start = drs.next_id;
    let (mut out, errors) = step_condition_body(ops, drs, condition);
    for r in &mut out.referents {
        if r.id >= start {
            r.from_condition = true;
        }
    }
    (out, errors)
}
fn step_condition_body(ops: &impl StepOps, drs: DRS, condition: &Condition) -> StepResult {
    match condition {
        Condition::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.condition(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        Condition::DuringPart {
            whose: Some(who), ..
        } => {
            let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, who);
            errors.extend(kind_check(
                Kind::Player,
                ids.first()
                    .and_then(|id| out.find_by_id(*id))
                    .map(|r| r.payload.kind()),
            ));
            (out, errors)
        }
        Condition::DuringPart { whose: None, .. } => (drs, Vec::new()),
        Condition::Exists { subject } => {
            let start = drs.next_id;
            let (mut out, ids, mut errors) = ops.noun(drs, GramRole::Subj, true, subject);
            for r in &mut out.referents {
                if r.id >= start && ids.contains(&r.id) {
                    r.existential_domain = true;
                }
            }
            errors.extend(refuse(
                subject.existential_mention(),
                Refusal::ExistentialMention,
            ));
            (out, errors)
        }
        Condition::Did { clause } => step_lookback_condition(ops, &drs, clause),
        Condition::GameHas { designation } => (
            drs,
            refuse(
                matches!(
                    designation,
                    DesignationLabel::Named {
                        scope: DesignationScope::HeldByGame,
                        ..
                    }
                ),
                Refusal::DesignationScope {
                    designation: designation.clone(),
                },
            ),
        ),
        Condition::Matches { subject, predicate } => step_matches(ops, drs, subject, predicate),
        Condition::CompareAmt { subject, bound, .. } => {
            let (out, mut errors) = ops.amount(drs, subject);
            let (mut out, more) = ops.amount(out, bound);
            errors.extend(more);
            out.prepend(
                vec![Referent::new(
                    Determiner::The,
                    Plurality::One,
                    Payload::Difference,
                )],
                GramRole::Other,
                false,
            );
            errors.extend(refuse(subject.read(), Refusal::ReadAmount));
            (out, errors)
        }
        Condition::VoteLead { .. } => {
            let found = drs.count_outcomes(&OutcomeSort::VoteHeld);
            (
                drs,
                refuse(
                    found == 1,
                    Refusal::OutcomeInScope {
                        s: OutcomeSort::VoteHeld,
                        found: found as u64,
                    },
                ),
            )
        }
        Condition::AnyResultIs { bound, .. } => {
            let actual = drs.count_outcomes(&OutcomeSort::RollResult);
            let found =
                if actual == 0 { drs.count_outcomes(&OutcomeSort::DiceRolled) } else { actual };
            let (out, mut errors) = ops.amount(drs, bound);
            errors.extend(refuse(
                found == 1,
                Refusal::OutcomeInScope {
                    s: OutcomeSort::RollResult,
                    found: found as u64,
                },
            ));
            (out, errors)
        }
        Condition::RolledDoubles => {
            let found = drs.count_outcomes(&OutcomeSort::RollResult);
            (
                drs,
                refuse(
                    found == 1,
                    Refusal::OutcomeInScope {
                        s: OutcomeSort::RollResult,
                        found: found as u64,
                    },
                ),
            )
        }
        Condition::HasPriority { player } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Subj, true, player);
            (out, errors)
        }
        Condition::SoChooses { chooser } => {
            let (out, ids, mut errors) = ops.noun(drs, GramRole::Subj, true, chooser);
            errors.extend(kind_check(
                Kind::Player,
                ids.first()
                    .and_then(|id| out.find_by_id(*id))
                    .map(|r| r.payload.kind()),
            ));
            (out, errors)
        }
        Condition::CouldBeginToCast {
            player,
            subject,
            from_,
        } => step_cast_condition(ops, drs, player, subject, from_),
        Condition::HeldSince { condition, since } => {
            let (out, mut errors) = ops.condition(drs, condition);
            let (out, more) = ops.event(out, since);
            errors.extend(more);
            (out, errors)
        }
        Condition::Not { condition } => step_negated_condition(ops, drs, condition),
        Condition::And { conjuncts } => step_conjunction(ops, drs, conjuncts),
        Condition::Or { disjuncts } => step_disjunction(ops, &drs, disjuncts),
    }
}
fn step_matches(
    ops: &impl StepOps,
    drs: DRS,
    subject: &NounPhrase,
    predicate: &Predicate,
) -> StepResult {
    let subject = if predicate.demands_card() {
        subject.narrow_bare_to(&SlotCarrier::Card)
    } else {
        subject.clone()
    };
    let start = drs.next_id;
    let (subject_context, ids, mut errors) = ops.noun(drs, GramRole::Subj, true, &subject);
    let actual = ids
        .first()
        .and_then(|id| subject_context.find_by_id(*id))
        .map(|r| r.payload.kind());
    let mut context = subject_context.clone();
    context.relative_kind = predicate.kind().or_else(|| actual.clone()).map(Box::new);
    context.relative_types.clear();
    context.relative_zone = None;
    context.relative_control = None;
    context.contrast_excluded.splice(0..0, ids.iter().copied());
    let (mut out, more) = ops.predicate(context, predicate);
    errors.extend(more);
    out.relative_types
        .clone_from(&subject_context.relative_types);
    out.relative_zone.clone_from(&subject_context.relative_zone);
    out.relative_control = subject_context.relative_control;
    out.contrast_excluded
        .clone_from(&subject_context.contrast_excluded);
    for r in &mut out.referents {
        if ids.contains(&r.id)
            && let Payload::Object { ty, .. } = r.payload.as_mut()
        {
            *ty = super::merge_types(ty, &predicate.seed_types());
        }
    }
    if let Some(expected) = predicate.kind() {
        errors.extend(kind_check(expected, actual));
    }
    let coordinated = matches!(
        subject,
        NounPhrase::Coord {
            coordinator: Coordinator::And | Coordinator::AndOr,
            ..
        }
    );
    let subject_ok = subject.player_group_mention()
        || ids.iter().all(|id| {
            *id < start
                || subject_context.find_by_id(*id).is_some_and(|r| {
                    matches!(r.det, Determiner::The | Determiner::Self_)
                        || (coordinated && r.det == Determiner::A)
                })
                || subject_context
                    .scope_stack
                    .first()
                    .is_some_and(|frame| frame.caller_referent_ids.contains(id))
        });
    errors.extend(refuse(subject_ok, Refusal::TestSubject));
    errors.extend(refuse(predicate.says(), Refusal::PredSays));
    (out, errors)
}
fn step_negated_condition(ops: &impl StepOps, drs: DRS, condition: &Condition) -> StepResult {
    if let Condition::Matches { subject, predicate } = condition {
        let subject = if predicate.demands_card() {
            subject.narrow_bare_to(&SlotCarrier::Card)
        } else {
            *subject.clone()
        };
        if subject.self_defined_ok() {
            let (subject_context, ids, mut errors) = ops.noun(drs, GramRole::Subj, true, &subject);
            let mut context = subject_context.clone();
            context.relative_kind = predicate.kind().map(Box::new);
            context.relative_types.clear();
            context.relative_zone = None;
            context.relative_control = None;
            context.contrast_excluded.splice(0..0, ids.iter().copied());
            let (_, more) = ops.predicate(context, predicate);
            errors.extend(more);
            if let Some(expected) = predicate.kind() {
                errors.extend(kind_check(
                    expected,
                    ids.first()
                        .and_then(|id| subject_context.find_by_id(*id))
                        .map(|r| r.payload.kind()),
                ));
            }
            return (subject_context, errors);
        }
    }
    let (_, errors) = ops.condition(drs.clone(), condition);
    (drs, errors)
}

fn step_lookback_condition(
    ops: &impl StepOps,
    drs: &DRS,
    clause: &deckmaste_semantics_v3::ast::DidClause,
) -> StepResult {
    let mut context = drs.clone();
    context.relative_binder = None;
    context.relative_uses = 0;
    let (mut out, mut errors) = super::step_did_with(ops, &context, clause);
    let grounded = matches!(clause.within.as_deref(), Some(LookbackSpan::ThisWay))
        && !errors.contains(&Refusal::ThisWayUngrounded);
    out.referents.retain(|r| !(grounded && r.id >= drs.next_id && r.plur == Plurality::One && matches!(r.payload.as_ref(), Payload::Outcome { sort } if drs.count_outcomes(sort) == 1)));
    out.relative_binder = drs.relative_binder;
    out.relative_uses = drs.relative_uses;
    errors.extend(refuse(clause.within.is_some(), Refusal::UnanchoredLookback));
    (out, errors)
}

fn step_conjunction(
    ops: &impl StepOps,
    mut drs: DRS,
    conjuncts: &[deckmaste_semantics_v3::ast::Condition],
) -> StepResult {
    let mut errors = refuse(conjuncts.len() >= 2, Refusal::AtLeastTwo);
    errors.extend(refuse(
        conjuncts.iter().all(|c| !c.is_and()),
        Refusal::FlatConjuncts,
    ));
    for part in conjuncts {
        drs.next_clause();
        let (out, more) = ops.condition(drs, part);
        drs = out;
        errors.extend(more);
    }
    (drs, errors)
}

fn step_disjunction(
    ops: &impl StepOps,
    drs: &DRS,
    disjuncts: &[deckmaste_semantics_v3::ast::Condition],
) -> StepResult {
    let mut errors = refuse(disjuncts.len() >= 2, Refusal::AtLeastTwo);
    errors.extend(refuse(
        disjuncts.iter().all(|c| !c.is_or()),
        Refusal::FlatDisjuncts,
    ));
    let mut branches = Vec::new();
    for part in disjuncts {
        let mut branch = drs.clone();
        branch.next_clause();
        let (out, more) = ops.condition(branch, part);
        branches.push(out);
        errors.extend(more);
    }
    (drs.common_header_context(&branches), errors)
}

fn step_cast_condition(
    ops: &impl StepOps,
    drs: DRS,
    player: &deckmaste_semantics_v3::ast::NounPhrase,
    subject: &deckmaste_semantics_v3::ast::NounPhrase,
    from_: &deckmaste_semantics_v3::ast::Location,
) -> StepResult {
    let (out, ids, mut errors) = ops.noun(drs, GramRole::Subj, true, player);
    errors.extend(kind_check(
        Kind::Player,
        ids.first()
            .and_then(|id| out.find_by_id(*id))
            .map(|r| r.payload.kind()),
    ));
    let (out, _, more) = ops.noun(out, GramRole::Obj, true, subject);
    errors.extend(more);
    let (out, more) = super::step_location_with(ops, out, from_);
    errors.extend(more);
    (out, errors)
}
