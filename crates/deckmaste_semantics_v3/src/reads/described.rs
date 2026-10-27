//! Stepping of definite, indefinite, and participial descriptions.
use super::DRS;
use super::Determiner;
use super::GramRole;
use super::NounResult;
use super::StepOps;
use super::circumstances::refuse;
use super::participial_read;
use super::step_determiner_with;
use crate::ast::ChoiceSort;
use crate::ast::DetPhrase;
use crate::ast::Kind;
use crate::ast::LookbackSpan;
use crate::ast::Plurality;
use crate::ast::Predicate;
use crate::refusal::Refusal;

pub fn step_description_with(
    ops: &impl StepOps,
    drs: &DRS,
    role: GramRole,
    in_arg_slot: bool,
    determiner: &DetPhrase,
    predicate: &Predicate,
) -> NounResult {
    if let (Some(plurality), Some((clause, anchor))) = (
        determiner.participial_number(),
        predicate.participial_anchor(),
    ) {
        let (mut context, mut errors) = step_determiner_with(ops, drs.clone(), determiner);
        context.relative_kind = predicate.kind().map(Box::new);
        context.participial_anchor_read = Some(anchor.clone());
        let (mut out, more) = ops.predicate(context, &predicate.with_anchor_span());
        errors.extend(more);
        out.participial_anchor_read
            .clone_from(&drs.participial_anchor_read);
        let (word, zones) =
            Predicate::participial_head(&predicate.conjuncts(), plurality.is_none());
        let mut read_from = drs.clone();
        if matches!(predicate, Predicate::InCaller { .. }) {
            read_from.enter_caller();
        }
        let found = read_from.participial_candidates(
            &anchor,
            plurality,
            word.as_ref(),
            clause.within.is_some(),
            &zones,
        );
        errors.extend(refuse(
            clause.participial_agent_ok(&anchor),
            Refusal::ParticipialAgent,
        ));
        match participial_read(plurality, &found) {
            Ok(read) => {
                let ids: Vec<_> = read.iter().map(|r| r.id).collect();
                for reference in read {
                    if out.participial_only.contains(reference) {
                        out.referents.retain(|r| r.id != reference.id);
                        out.referents.insert(0, reference.clone());
                        out.participial_only.retain(|r| r != reference);
                    }
                    if let Some(r) = out.referents.iter_mut().find(|r| r.id == reference.id) {
                        r.linked = false;
                    }
                    out.record_mention(reference.id, role, in_arg_slot, false);
                }
                (out, ids, errors)
            }
            Err(found) => {
                if found == 0
                    && matches!(clause.within.as_deref(), Some(LookbackSpan::ThisWay))
                    && drs
                        .caused_events
                        .iter()
                        .any(|event| event.permitted && event.read_by(Some(&anchor), &[]))
                {
                    let reference = described_binding(determiner, predicate);
                    let ids = out.prepend(vec![reference], role, in_arg_slot);
                    (out, ids, errors)
                } else {
                    errors.push(Refusal::ParticipialDescription {
                        anchor,
                        found: found as u64,
                    });
                    (out, Vec::new(), errors)
                }
            }
        }
    } else {
        let (mut context, mut errors) = step_determiner_with(ops, drs.clone(), determiner);
        context.relative_kind = predicate.kind().map(Box::new);
        context.relative_types.clear();
        context.relative_zone = None;
        context.relative_control = None;
        context.contrast_excluded.clear();
        let (mut out, more) = ops.predicate(context, predicate);
        errors.extend(more);
        out.relative_types.clone_from(&drs.relative_types);
        out.relative_zone.clone_from(&drs.relative_zone);
        out.relative_control = drs.relative_control;
        out.contrast_excluded.clone_from(&drs.contrast_excluded);
        let mut read_from = drs.clone();
        if matches!(predicate, Predicate::InCaller { .. }) {
            read_from.enter_caller();
        }
        let kind = predicate.kind().unwrap_or(Kind::Object);
        let chosen = match (determiner, predicate.chosen_ref()) {
            (DetPhrase::The { plur }, Some(reference)) if ChoiceSort::at(&kind).is_none() => {
                Some((plur, reference))
            }
            _ => None,
        };
        let found = chosen.map_or_else(Vec::new, |(plurality, _)| {
            read_from.chosen_antecedents(plurality, predicate)
        });
        let chosen_errors = match chosen {
            Some((Plurality::One, reference))
                if !reference.permits(found.len())
                    && read_from.count_chosen_objects(&kind) != 0 =>
            {
                vec![Refusal::ChosenObjectRef {
                    ref_: reference.clone(),
                    found: found.len() as u64,
                }]
            }
            _ => Vec::new(),
        };
        let antecedents = match chosen {
            Some((_, reference)) => {
                if reference.permits(found.len()) {
                    found.last().copied().into_iter().collect()
                } else {
                    Vec::new()
                }
            }
            None if matches!(
                determiner,
                DetPhrase::The {
                    plur: Plurality::One
                }
            ) && !predicate.uniquifies() =>
            {
                read_from.definite_antecedents(predicate)
            }
            None => Vec::new(),
        };
        if let [antecedent] = antecedents.as_slice() {
            out.record_mention(antecedent.id, role, in_arg_slot, false);
            (out, vec![antecedent.id], errors)
        } else {
            let ids = out.prepend(
                vec![described_binding(determiner, predicate)],
                role,
                in_arg_slot,
            );
            if determiner.determiner() == Determiner::Target {
                errors.extend(refuse(
                    kind.targetable(),
                    Refusal::Targetable { k: Box::new(kind) },
                ));
            }
            errors.extend(chosen_errors);
            errors.extend(refuse(
                determiner.determiner() != Determiner::The || predicate.uniquifies(),
                Refusal::Uniquifying,
            ));
            (out, ids, errors)
        }
    }
}
fn described_binding(determiner: &DetPhrase, predicate: &Predicate) -> super::Referent {
    let mut reference = predicate.binding(
        determiner.determiner(),
        determiner.plurality(),
        &predicate.kind().unwrap_or(Kind::Object),
    );
    reference.controlled_by_you = predicate.control_fact();
    reference.opponent_controls = predicate.opponent_controls();
    reference
}
