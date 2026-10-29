//! Grounded and relative lookbacks over caused and triggering events.
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::DidClause;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::LookbackSpan;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Subject;

use super::ActionClause;
use super::DRS;
use super::GramRole;
use super::Referent;
use super::StepOps;
use super::StepPhase;
use super::StepResult;
use super::circumstances::refuse;
use super::participial_read;
use crate::reads::ActionAnchorsExt;
use crate::reads::ActionEventHistoryExt;
use crate::reads::ActionEventsExt;
use crate::reads::DidClauseAnchorsExt;
use crate::reads::NounPhraseFactsExt;
use crate::refusal::Refusal;

#[must_use]
pub fn lookback_ground_refusals(
    drs: &DRS,
    action: &Action,
    subject: Option<&Kind>,
    within: Option<&LookbackSpan>,
) -> Vec<Refusal> {
    match within {
        Some(LookbackSpan::ThisWay) => {
            let anchor = action.participial_anchor();
            refuse(
                (drs.participial_anchor_read.is_some() && drs.participial_anchor_read == anchor)
                    || !drs.this_way_events(action, subject).is_empty(),
                Refusal::ThisWayUngrounded,
            )
        }
        Some(LookbackSpan::Triggering) => {
            let anchor = action.participial_anchor();
            let verbs = action.event_verbs();
            refuse(
                drs.trigger_events.iter().any(|event| {
                    event.read_by(anchor.as_ref(), &verbs)
                        && event.subject_fits(subject, action.subject_is_patient())
                }),
                Refusal::TriggeringUngrounded,
            )
        }
        _ => Vec::new(),
    }
}
pub fn step_did_with(ops: &impl StepOps, drs: &DRS, clause: &DidClause) -> StepResult {
    let recorded =
        drs.this_way_recorded_origin(&clause.subj, &clause.action, clause.within.as_deref());
    let mut subject_context = drs.clone();
    if clause
        .action
        .subject_reads_departed_permanent(recorded.as_ref())
    {
        subject_context.reads_last_known = true;
        subject_context.historical_referent_ids = drs.referents.iter().map(|r| r.id).collect();
    }
    let hidden = match clause.subj.as_ref() {
        Subject::Actor { .. } => Vec::new(),
        Subject::Any | Subject::By { .. } => drs.performers.clone(),
    };
    let historical = |mut context: DRS| {
        if !hidden.is_empty() {
            context.performers.clear();
        }
        context.action_pattern = true;
        context.historical_referent_ids = drs.referents.iter().map(|r| r.id).collect();
        context.reads_last_known = drs.reads_last_known;
        context.handed_origin = recorded.or_else(|| drs.handed_origin.clone());
        context.participial_anchor_read = None;
        context
    };
    let restored = |mut context: DRS| {
        let subject_kind = match clause.subj.as_ref() {
            Subject::Actor { .. } => context.performers.first().map(|actor| *actor.kind.clone()),
            Subject::By { np } => Some(np.kind_in(drs, Kind::Object)),
            Subject::Any => None,
        };
        context.reads_last_known = drs.reads_last_known;
        context.handed_origin.clone_from(&drs.handed_origin);
        (
            context,
            lookback_ground_refusals(
                drs,
                &clause.action,
                subject_kind.as_ref(),
                clause.within.as_deref(),
            ),
        )
    };
    let finish = |context| match &clause.within {
        Some(span) => super::step_lookback_span_with(ops, context, span),
        None => (context, Vec::new()),
    };
    let (mut out, mut errors) = ActionClause {
        subject: &clause.subj,
        action: &clause.action,
        phase: StepPhase::PostEvent,
        gate: Action::event_head_ok,
        event: true,
    }
    .step_context_with(ops, &subject_context, historical, restored, finish);
    out.performers.splice(0..0, hidden);
    out.historical_referent_ids
        .clone_from(&drs.historical_referent_ids);
    out.participial_anchor_read
        .clone_from(&drs.participial_anchor_read);
    if let Some(ordinal) = &clause.ordinal {
        errors.extend(super::event_ordinal_refusals(
            &ordinal.ordinal,
            ordinal.per.as_ref(),
        ));
    }
    (out, errors)
}
/// Discourse inspection of `DidClause`.
pub trait DidClauseLookbacksExt: Sized {
    #[must_use]
    fn by_source(&self) -> bool;
    #[must_use]
    fn span_refusals(&self, drs: &DRS) -> Vec<Refusal>;
}

impl DidClauseLookbacksExt for DidClause {
    fn by_source(&self) -> bool {
        matches!(self.subj.as_ref(), Subject::By { np } if **np == NounPhrase::This || matches!(np.as_ref(), NounPhrase::Headed { subject, .. } if **subject == NounPhrase::This))
    }
    fn span_refusals(&self, drs: &DRS) -> Vec<Refusal> {
        if self.within.is_some() {
            return Vec::new();
        }
        if let (Some(anchor), Some(linked)) = (self.anchor(), &drs.keyword_linked)
            && self.by_source()
        {
            let context = DRS {
                referents: linked.clone(),
                ..DRS::default()
            };
            let found =
                context.participial_candidates(&anchor, Some(&Plurality::One), None, false, &[]);
            match participial_read(Some(&Plurality::One), &found) {
                Ok(_) => Vec::new(),
                Err(found) => vec![Refusal::ParticipialDescription {
                    anchor,
                    found: found as u64,
                }],
            }
        } else {
            vec![Refusal::UnanchoredLookback]
        }
    }
}
pub fn step_relative_did_with(
    ops: &impl StepOps,
    drs: &DRS,
    which: Referent,
    clause: &DidClause,
) -> StepResult {
    let mut context = drs.clone();
    context.next_clause();
    let binder_id = context.prepend(vec![which], GramRole::Other, false)[0];
    context.relative_binder = Some(binder_id);
    context.relative_uses = 0;
    let (out, mut errors) = step_did_with(ops, &context, clause);
    errors.extend(refuse(out.relative_uses != 0, Refusal::WhichUnused));
    errors.extend(clause.span_refusals(drs));
    (DRS::leave_relative(drs, &out), errors)
}
