//! Shared action-clause spine for events, lookbacks, and deontic rules.
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Causing;
use deckmaste_semantics_v3::ast::CreationSpec;
use deckmaste_semantics_v3::ast::GameEvent;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::Ordinal;
use deckmaste_semantics_v3::ast::Period;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Subject;
use deckmaste_semantics_v3::ast::TokenSpec;
use deckmaste_semantics_v3::ast::TurnPart;
use deckmaste_semantics_v3::ast::WouldClause;

use super::ActorShape;
use super::DRS;
use super::Determiner;
use super::GramRole;
use super::StepOps;
use super::StepPhase;
use super::StepResult;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::reads::ActionEventsExt;
use crate::reads::KindWordsExt;
use crate::reads::NounPhraseEventsExt;
use crate::reads::NounPhraseFactsExt;
use crate::reads::NounPhrasePhraseFactsExt;
use crate::reads::SubjectAnchorsExt;
use crate::refusal::Refusal;

/// Discourse inspection of `Action`.
pub trait ActionClausesExt: Sized {
    #[must_use]
    fn clause_actor_kind(&self) -> Option<Kind>;
}

impl ActionClausesExt for Action {
    fn clause_actor_kind(&self) -> Option<Kind> {
        match self.scope_body() {
            Self::Draw { .. }
            | Self::Procedure { .. }
            | Self::FlipCoins { .. }
            | Self::RollDice { .. }
            | Self::CreateObject { .. }
            | Self::PutCounters { .. }
            | Self::RemoveCounters { .. }
            | Self::MoveCounters { .. } => Some(Kind::Player),
            Self::DealDamage { .. } => Some(Kind::Object),
            _ => None,
        }
    }
}
/// Discourse inspection of `TokenSpec`.
pub trait TokenSpecClausesExt: Sized {
    #[must_use]
    fn event_source_ok(&self) -> bool;
}

impl TokenSpecClausesExt for TokenSpec {
    fn event_source_ok(&self) -> bool {
        let mut token = self;
        for _ in 0..64 {
            match token {
                Self::InCaller { body } => token = body,
                Self::CopyOf { source, .. } => return source.token_phrase(),
                _ => return true,
            }
        }
        false
    }
}
fn event_voice_checks(drs: &DRS, subject: &Subject, action: &Action) -> Vec<Refusal> {
    let agent = match subject {
        Subject::Any => None,
        Subject::Actor { np } | Subject::By { np } => Some(np),
    };
    let agent_read = agent.is_none_or(|np| np.event_agent_ok());
    match action.scope_body() {
        Action::PutCounters { .. }
        | Action::RemoveCounters { .. }
        | Action::MoveCounters { .. } => refuse(agent_read, Refusal::EventAgent),
        Action::CreateObject { spec, under, .. } => {
            let mut errors = kind_check(
                Kind::Player,
                under.as_ref().map(|np| np.kind_in(drs, Kind::Player)),
            );
            if let CreationSpec::Token { spec, .. } = spec.as_ref() {
                errors.extend(refuse(spec.event_source_ok(), Refusal::TokenPhrase));
            }
            errors.extend(refuse(
                agent_read
                    && under
                        .as_ref()
                        .is_none_or(|np| np.plurality() == Plurality::One && np.event_agent_ok()),
                Refusal::VerbedVoiceOk,
            ));
            errors
        }
        _ => Vec::new(),
    }
}
fn step_subject_before(ops: &impl StepOps, drs: DRS, subject: &Subject) -> StepResult {
    if let Subject::Actor { np } = subject {
        let (mut out, ids, errors) = ops.noun(drs, GramRole::Subj, true, np);
        let kind = ids
            .first()
            .and_then(|id| out.find_by_id(*id))
            .map_or_else(|| np.kind_in(&out, Kind::Player), |r| r.payload.kind());
        out.performers.insert(
            0,
            ActorShape {
                kind: Box::new(kind),
                is_you: np.is_you(),
                ascribable: false,
                ref_ids: ids,
                plur: np.plurality(),
                selves: Vec::new(),
            },
        );
        (out, errors)
    } else {
        (drs, Vec::new())
    }
}
pub fn step_event_subject_with(
    ops: &impl StepOps,
    drs: DRS,
    subject: &Subject,
    phase: StepPhase,
    action: &Action,
) -> StepResult {
    if matches!(subject, Subject::Actor { np } if np.has_bare_other()) {
        let mut preview = drs.clone();
        preview.action_pattern = true;
        let after = ops.action(phase, subject.patient_role(), preview, action).0;
        let ahead = after
            .referents
            .into_iter()
            .filter(|r| {
                r.id >= drs.next_id && matches!(r.det, Determiner::Target | Determiner::The)
            })
            .rev()
            .collect();
        let saved = drs.clause_participants.clone();
        let mut context = drs;
        context.clause_participants = ahead;
        let (mut out, errors) = step_subject_before(ops, context, subject);
        out.clause_participants = saved;
        (out, errors)
    } else {
        step_subject_before(ops, drs, subject)
    }
}
pub fn step_subject_after_with(
    ops: &impl StepOps,
    mut drs: DRS,
    subject: &Subject,
    expected: Option<Kind>,
    targeter: bool,
) -> StepResult {
    match subject {
        Subject::Actor { .. } => {
            if !drs.performers.is_empty() {
                drs.performers.remove(0);
            }
            (drs, Vec::new())
        }
        Subject::Any => (drs, Vec::new()),
        Subject::By { np } => {
            let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, true, np);
            let actual = ids
                .first()
                .and_then(|id| out.find_by_id(*id))
                .map(|r| r.payload.kind());
            if let Some(expected) = expected {
                errors.extend(kind_check(expected, actual.clone()));
            }
            if targeter && let Some(kind) = actual {
                errors.extend(refuse(
                    kind.targeter(),
                    Refusal::Targeter { k: Box::new(kind) },
                ));
            }
            (out, errors)
        }
    }
}

pub struct ActionClause<'a> {
    pub subject: &'a Subject,
    pub action: &'a Action,
    pub phase: StepPhase,
    pub gate: fn(&Action) -> bool,
    pub event: bool,
}
impl ActionClause<'_> {
    pub fn step_with(&self, ops: &impl StepOps, drs: &DRS) -> StepResult {
        self.step_context_with(
            ops,
            drs,
            |mut d| {
                d.action_pattern = true;
                d
            },
            |d| (d, Vec::new()),
            |d| (d, Vec::new()),
        )
    }
    pub fn step_context_with(
        &self,
        ops: &impl StepOps,
        drs: &DRS,
        enter: impl FnOnce(DRS) -> DRS,
        between: impl FnOnce(DRS) -> StepResult,
        finish: impl FnOnce(DRS) -> StepResult,
    ) -> StepResult {
        let mut errors = refuse((self.gate)(self.action), Refusal::EventHead);
        if self.event
            && let Action::Procedure { subject, .. } = self.action.scope_body()
        {
            errors.extend(refuse(
                subject.plurality() == Plurality::One,
                Refusal::Singular,
            ));
        }
        let (out, more) =
            step_event_subject_with(ops, drs.clone(), self.subject, self.phase, self.action);
        errors.extend(more);
        if !matches!(self.action.scope_body(), Action::Draw { .. })
            && let (Some(expected), Some(shape)) =
                (self.action.clause_actor_kind(), out.performers.first())
        {
            errors.extend(kind_check(expected, Some(*shape.kind.clone())));
        }
        let (out, more) = ops.action(
            self.phase,
            self.subject.patient_role(),
            enter(out),
            self.action,
        );
        errors.extend(more);
        let (out, more) = between(out);
        errors.extend(more);
        let (out, more) = step_subject_after_with(
            ops,
            out,
            self.subject,
            self.action.clause_actor_kind(),
            matches!(self.action.scope_body(), Action::Target { .. }),
        );
        errors.extend(more);
        let (mut out, more) = finish(out);
        errors.extend(more);
        out.action_pattern = drs.action_pattern;
        if self.event {
            errors.extend(event_voice_checks(drs, self.subject, self.action));
        }
        (out, errors)
    }
}
#[must_use]
pub fn event_ordinal_refusals(ordinal: &Ordinal, per: Option<&Period>) -> Vec<Refusal> {
    let Ordinal::Nth { n } = ordinal;
    let mut errors = refuse(*n != 0, Refusal::OrdinalNonZero);
    if let Some(period) = per {
        errors.extend(refuse(
            !matches!(
                period,
                Period::ThisObject
                    | Period::Part {
                        part: TurnPart::Turn
                    }
            ),
            Refusal::EventOrdinalPeriod {
                period: period.clone(),
            },
        ));
    }
    errors
}
pub fn step_causing_with(ops: &impl StepOps, drs: DRS, cause: &Causing) -> StepResult {
    match cause {
        Causing::Source { source } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Other, false, source);
            (out, errors)
        }
        Causing::Event { event } => ops.event(drs, event),
        Causing::AnEffect => (drs, Vec::new()),
    }
}
pub fn step_would_with(ops: &impl StepOps, drs: DRS, clause: &WouldClause) -> StepResult {
    if ops.fuel() == 0 {
        return (drs, vec![Refusal::RecursionLimit]);
    }
    match clause {
        WouldClause::Would { subj, action } => ActionClause {
            subject: subj,
            action,
            phase: StepPhase::PreEvent,
            gate: Action::event_head_ok,
            event: true,
        }
        .step_with(ops, &drs),
        WouldClause::WouldAny { clauses } => {
            let mut branches = Vec::new();
            let mut errors = Vec::new();
            for clause in clauses {
                let (out, more) = ops.would(drs.clone(), clause);
                branches.push(out);
                errors.extend(more);
            }
            (drs.common_header_context(&branches), errors)
        }
        WouldClause::CausedBy { cause, clause } => {
            let (out, mut errors) = step_causing_with(ops, drs, cause);
            let (out, more) = ops.would(out, clause);
            errors.extend(more);
            (out, errors)
        }
        WouldClause::NthOccurrence {
            ordinal,
            per,
            clause,
        } => {
            let (out, mut errors) = ops.would(drs, clause);
            errors.extend(event_ordinal_refusals(ordinal, per.as_ref()));
            (out, errors)
        }
    }
}
pub fn step_event_with(ops: &impl StepOps, drs: DRS, event: &GameEvent) -> StepResult {
    match event {
        GameEvent::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.event(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        GameEvent::Does { subj, action } => ActionClause {
            subject: subj,
            action,
            phase: StepPhase::PostEvent,
            gate: Action::event_head_ok,
            event: true,
        }
        .step_with(ops, &drs),
        GameEvent::BeginningOf { whose, which, .. } | GameEvent::EndOf { whose, which, .. } => {
            let (out, mut errors) = if let Some(owner) = whose {
                let (out, _, mut errors) = ops.noun(drs, GramRole::Subj, false, owner);
                errors.extend(kind_check(Kind::Player, owner.kind()));
                (out, errors)
            } else {
                (drs, Vec::new())
            };
            let (out, more) = super::step_turn_ref_with(ops, out, which);
            errors.extend(more);
            (out, errors)
        }
        GameEvent::ChapterMark { chapters } => (
            drs,
            refuse(
                !chapters.is_empty()
                    && chapters
                        .iter()
                        .enumerate()
                        .all(|(i, n)| *n != 0 && !chapters[..i].contains(n)),
                Refusal::ChapterMarks,
            ),
        ),
        GameEvent::Now | GameEvent::StateBasedActionsChecked => (drs, Vec::new()),
        GameEvent::StateHolds { condition } => ops.condition(drs, condition),
        GameEvent::NthOccurrence {
            ordinal,
            per,
            event,
        } => {
            let (out, mut errors) = ops.event(drs, event);
            errors.extend(event_ordinal_refusals(ordinal, per.as_ref()));
            (out, errors)
        }
        GameEvent::Causes { cause, event } => {
            let (out, mut errors) = step_causing_with(ops, drs, cause);
            let (out, more) = ops.event(out, event);
            errors.extend(more);
            (out, errors)
        }
    }
}
