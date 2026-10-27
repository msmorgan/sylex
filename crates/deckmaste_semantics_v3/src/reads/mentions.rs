//! Identity-preserving mentions and noun coordination.
use super::DRS;
use super::Determiner;
use super::GramRole;
use super::NounResult;
use super::Payload;
use super::Referent;
use super::ReferentId;
use super::StepOps;
use super::circumstances::refuse;
use crate::ast::ChoiceMode;
use crate::ast::DetPhrase;
use crate::ast::Kind;
use crate::ast::NounPhrase;
use crate::ast::Plurality;
use crate::ast::SliceCount;
use crate::refusal::Refusal;

#[must_use]
pub fn mention_you(
    mut drs: DRS,
    plurality: &Plurality,
    role: GramRole,
    in_arg_slot: bool,
) -> NounResult {
    let existing = drs
        .visible_referents()
        .find(|r| {
            r.det == Determiner::Self_
                && r.plur == *plurality
                && *r.payload == Payload::Player { chosen: false }
                && !r.linked
        })
        .map(|r| r.id);
    let ids = if let Some(id) = existing {
        drs.record_mention(id, role, in_arg_slot, false);
        vec![id]
    } else {
        let mut r = Referent::new(
            Determiner::Self_,
            plurality.clone(),
            Payload::Player { chosen: false },
        );
        r.default_anchor = true;
        drs.prepend(vec![r], role, in_arg_slot)
    };
    (drs, ids, Vec::new())
}
#[must_use]
pub fn mention_this(mut drs: DRS, role: GramRole, in_arg_slot: bool) -> NounResult {
    let errors = refuse(!drs.sourceless, Refusal::NoSource);
    let existing = drs
        .visible_referents()
        .find(|r| {
            r.det == Determiner::Self_
                && r.default_anchor
                && r.payload.kind() == Kind::Object
                && !r.linked
        })
        .map(|r| r.id);
    let ids = if let Some(id) = existing {
        drs.record_mention(id, role, in_arg_slot, false);
        vec![id]
    } else {
        let mut r = Referent::new(
            Determiner::Self_,
            Plurality::One,
            Payload::Object {
                ty: drs.source_types.clone(),
                zone: None,
                prov: None,
                orig: None,
                size: None,
            },
        );
        r.default_anchor = true;
        drs.prepend(vec![r], role, in_arg_slot)
    };
    (drs, ids, errors)
}
#[must_use]
pub fn choice_mode_checks(drs: &DRS, mode: &ChoiceMode) -> Vec<Refusal> {
    if *mode != ChoiceMode::TheirChoice {
        return Vec::new();
    }
    let count = if let Some(actor) = drs.performers.first() {
        if *actor.kind == Kind::Player {
            let mut ids = actor.ref_ids.clone();
            ids.sort_unstable();
            ids.dedup();
            ids.iter()
                .filter_map(|id| drs.find_by_id(*id))
                .filter(|r| r.payload.kind() == Kind::Player)
                .count()
        } else {
            0
        }
    } else {
        drs.count_ones(&Kind::Player) + drs.count_many(&Kind::Player)
    };
    refuse(
        count == 1,
        Refusal::ChooserInScope {
            found: count as u64,
        },
    )
}
pub fn step_determiner_with(ops: &impl StepOps, drs: DRS, value: &DetPhrase) -> super::StepResult {
    match value {
        DetPhrase::Target { quantity } | DetPhrase::Count { quantity, .. } => {
            let mode_errors = match value {
                DetPhrase::Count {
                    mode: Some(mode), ..
                } => choice_mode_checks(&drs, mode),
                _ => Vec::new(),
            };
            let (out, mut errors) = super::step_quantity_with(ops, drs, quantity);
            errors.splice(0..0, refuse(quantity.non_zero(), Refusal::NonZeroQ));
            errors.extend(mode_errors);
            (out, errors)
        }
        DetPhrase::A { mode } => {
            let errors = choice_mode_checks(&drs, mode);
            (drs, errors)
        }
        DetPhrase::Each | DetPhrase::All | DetPhrase::The { .. } => (drs, Vec::new()),
    }
}
pub fn step_slice_count_with(
    ops: &impl StepOps,
    drs: DRS,
    value: &SliceCount,
) -> super::StepResult {
    match value {
        SliceCount::Counted { quantity } => super::step_quantity_with(ops, drs, quantity),
        SliceCount::Whole => (drs, Vec::new()),
        SliceCount::Totalling { bound, .. } => ops.amount(drs, bound),
    }
}
pub fn step_noun_alternatives_with(
    ops: &impl StepOps,
    drs: &DRS,
    role: GramRole,
    in_arg_slot: bool,
    alternatives: &[NounPhrase],
) -> NounResult {
    let mut out = drs.clone();
    let mut ids = Vec::new();
    let mut errors = refuse(alternatives.len() >= 2, Refusal::AtLeastTwo);
    let mut branches = Vec::new();
    for phrase in alternatives {
        let mut branch = drs.clone();
        branch.next_id = out.next_id;
        branch.coordinated_self = out.coordinated_self_in(&ids);
        let (result, branch_ids, branch_errors) = ops.noun(branch, role, in_arg_slot, phrase);
        let mut retained: Vec<_> = out
            .referents
            .iter()
            .filter(|r| r.id >= drs.next_id)
            .cloned()
            .collect();
        retained.extend(
            result
                .referents
                .iter()
                .filter(|r| r.id < drs.next_id)
                .map(|r| {
                    let mut r = r.clone();
                    if let Some(prior) = out.find_by_id(r.id) {
                        r.pronoun_resolved |= prior.pronoun_resolved;
                        let mut idents = prior.idents.clone();
                        for ident in &r.idents {
                            if !idents.contains(ident) {
                                idents.push(*ident);
                            }
                        }
                        r.idents = idents;
                    }
                    r
                }),
        );
        let mut merged = result.clone();
        merged.referents = retained;
        let merge = |left: &[ReferentId], right: &[ReferentId]| {
            let mut both = left.to_vec();
            for id in right {
                if !both.contains(id) {
                    both.push(*id);
                }
            }
            both
        };
        merged.pending_roles.clone_from(&out.pending_roles);
        for mention in &result.pending_roles {
            if !merged.pending_roles.contains(mention) {
                merged.pending_roles.push(*mention);
            }
        }
        merged.resolved_ids = merge(&out.resolved_ids, &result.resolved_ids);
        merged.relative_uses =
            out.relative_uses + result.relative_uses.saturating_sub(drs.relative_uses);
        merged.coordinated_self = drs.coordinated_self;
        out = merged;
        ids.extend(branch_ids);
        errors.extend(branch_errors);
        branches.push(result);
    }
    let common: Vec<_> = drs
        .common_header_context(&branches)
        .referents
        .into_iter()
        .filter(|r| r.id >= drs.next_id)
        .collect();
    let kept = |id: ReferentId| id < drs.next_id || common.iter().any(|r| r.id == id);
    out.pending_roles.retain(|(id, _)| kept(*id));
    out.resolved_ids.retain(|id| kept(*id));
    ids.retain(|id| kept(*id));
    let mut retained = common;
    retained.extend(out.referents.into_iter().filter(|r| r.id < drs.next_id));
    out.referents = retained;
    (out, ids, errors)
}
