//! Predicate stepping, including relative and contrast-anchor clauses.
use deckmaste_semantics_v3::ast::AggregateOp;
use deckmaste_semantics_v3::ast::AttachWord;
use deckmaste_semantics_v3::ast::AttachmentSide;
use deckmaste_semantics_v3::ast::ChoiceRef;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::CombatRelation;
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::NameSource;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::QualitySort;

use super::DRS;
use super::Determiner;
use super::GramRole;
use super::Payload;
use super::Referent;
use super::StepOps;
use super::StepResult;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::reads::ChoiceRefWordsExt;
use crate::reads::ChoiceSortWordsExt;
use crate::reads::DesignationLabelPredicateFactsExt;
use crate::reads::KeywordTermChoiceDomainsExt;
use crate::reads::KindResolutionExt;
use crate::reads::KindWordsExt;
use crate::reads::NounPhraseEventsExt;
use crate::reads::NounPhraseFactsExt;
use crate::reads::NounPhrasePhraseFactsExt;
use crate::reads::PredicateFactsExt;
use crate::reads::ProjAxisPredicateFactsExt;
use crate::reads::QualitySortWordsExt;
use crate::refusal::Refusal;
fn head_kind(drs: &DRS) -> Kind {
    drs.relative_kind
        .as_deref()
        .cloned()
        .unwrap_or(Kind::Object)
}
fn either() -> Kind {
    Kind::Join {
        left: Box::new(Kind::Object),
        right: Box::new(Kind::Player),
    }
}
fn choice_errors(drs: &DRS, reference: &ChoiceRef, sort: ChoiceSort) -> Vec<Refusal> {
    let found = drs.count_choice(&sort);
    refuse(
        reference.permits(found),
        Refusal::ChoiceRef {
            ref_: reference.clone(),
            sort,
            found: found as u64,
        },
    )
}
fn attachment_host_check(word: Option<&AttachWord>, kind: &Kind) -> Vec<Refusal> {
    let mut errors = refuse(
        kind.lte(&either()),
        Refusal::KindLte {
            k: Box::new(kind.clone()),
            bound: Box::new(either()),
        },
    );
    errors.extend(refuse(
        word.is_none_or(|word| *word == AttachWord::Enchanted || kind.lte(&Kind::Object)),
        Refusal::KindLte {
            k: Box::new(kind.clone()),
            bound: Box::new(Kind::Object),
        },
    ));
    errors
}
pub fn step_predicate_with(ops: &impl StepOps, mut drs: DRS, predicate: &Predicate) -> StepResult {
    match predicate {
        Predicate::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.predicate(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        Predicate::Not { predicate } => {
            let (out, mut errors) = ops.predicate(drs.clone(), predicate);
            drs.next_id = out.next_id;
            errors.extend(refuse(predicate.negatable(), Refusal::Negatable));
            (drs, errors)
        }
        Predicate::Chosen { ref_ } => step_chosen(drs, ref_),
        Predicate::QualityNoun { sort, domain } => super::sorted_domain_check_with(
            ops,
            drs,
            &ChoiceSort::Quality { sort: sort.clone() },
            domain.as_deref(),
        ),
        Predicate::OfYourChoice { sort, domain } => {
            step_quality_choice(ops, drs, sort, domain.as_deref())
        }
        Predicate::EachKind { domain, .. } => step_each_kind(ops, drs, domain.as_deref()),
        Predicate::OfChosen { ref_, sort } => step_chosen_quality(drs, ref_, sort),
        Predicate::HasCounters { .. } => step_counter_holder(drs),
        Predicate::HasPossessor { possessor, .. } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, possessor);
            errors.extend(refuse(possessor.sole_holder_ok(), Refusal::SoleHolder));
            (out, errors)
        }
        Predicate::IsOpponentOf { who } | Predicate::IsTeammateOf { who } => {
            step_player_relation(ops, drs, who)
        }
        Predicate::Did { clause } => {
            super::step_relative_did_with(ops, &drs, head_kind(&drs).binding(), clause)
        }
        Predicate::Compare { axes, bound, .. } => step_compare(ops, drs, axes, bound),
        Predicate::Superlative { op, axis, domain } => step_superlative(ops, drs, op, axis, domain),
        Predicate::WithMostVotes => step_most_votes(drs),
        Predicate::ChoseExtreme { .. } => step_chose_extreme(drs),
        Predicate::CompareOver {
            domain,
            measure,
            bound,
            ..
        } => step_compare_over(ops, &drs, domain, measure, bound),
        Predicate::Attachment {
            side,
            word,
            counterpart,
        } => step_attachment(ops, drs, side, word.as_ref(), counterpart.as_deref()),
        Predicate::In { loc } => super::step_location_with(ops, drs, loc),
        Predicate::HasDesignation {
            designation,
            holder,
        } => step_designation(ops, drs, designation, holder.as_deref()),
        Predicate::HeldBeforeCast { designation } => (
            drs,
            refuse(
                designation.holder() == Some(Kind::Object),
                Refusal::DesignationHolder {
                    designation: designation.clone(),
                    k: Box::new(Kind::Object),
                },
            ),
        ),
        Predicate::InCombat {
            relation,
            counterpart: Some(counterpart),
        } => step_combat_predicate(ops, drs, relation, counterpart),
        Predicate::Related {
            counterpart: Some(source),
            ..
        }
        | Predicate::CounterKindOn { subject: source }
        | Predicate::AbilityOf { source }
        | Predicate::LegalAttachment {
            counterpart: source,
            ..
        } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Other, false, source);
            (out, errors)
        }
        Predicate::Shares { with_, .. } => step_shares(ops, drs, with_),
        Predicate::Named {
            source: NameSource::Chosen,
        } => step_chosen_name(drs),
        Predicate::HasKeyword { keyword } => {
            (drs, refuse(keyword.known(), Refusal::KnownKeywordTerm))
        }
        Predicate::Other => step_other(drs),
        Predicate::NotChosen => {
            let kind = head_kind(&drs);
            let ok = drs.count_parts(&kind) != 0 || drs.choice_was_possible(&kind);
            (
                drs,
                refuse(ok, Refusal::ChoiceInScope { k: Box::new(kind) }),
            )
        }
        Predicate::OtherThan { anchor } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, anchor);
            errors.extend(refuse(
                anchor.anchor_phrase() && anchor.plurality() == Plurality::One,
                Refusal::ComplementAnchor,
            ));
            (out, errors)
        }
        Predicate::Targets { subject, .. } => step_targets(ops, drs, subject),
        Predicate::CoinCameUp { .. } => step_coin_outcome(drs),
        Predicate::Coord {
            coordinator: Coordinator::And,
            items,
        } => step_predicate_conjunction(ops, &drs, predicate, items),
        Predicate::Coord { items, .. } => step_predicate_alternatives(ops, drs, predicate, items),
        Predicate::HasType { .. }
        | Predicate::HasSubtype { .. }
        | Predicate::HasSupertype { .. }
        | Predicate::IsPlayer
        | Predicate::ColorIs { .. }
        | Predicate::ColorCount { .. }
        | Predicate::Named { .. }
        | Predicate::InCombat {
            counterpart: None, ..
        }
        | Predicate::Related {
            counterpart: None, ..
        }
        | Predicate::IsCard
        | Predicate::IsToken
        | Predicate::IsEmblem
        | Predicate::IsCopyOfACard
        | Predicate::HasTypeIgnoringTypeChanges { .. }
        | Predicate::CurrentFace { .. }
        | Predicate::HasStatus { .. }
        | Predicate::IsSource
        | Predicate::ManaCostHas { .. }
        | Predicate::AbilityHead { .. }
        | Predicate::ActivationCostHas { .. }
        | Predicate::TriggeredNotLeftStack => (drs, Vec::new()),
    }
}
fn step_compare_over(
    ops: &impl StepOps,
    drs: &DRS,
    domain: &Predicate,
    measure: &deckmaste_semantics_v3::ast::Amount,
    bound: &deckmaste_semantics_v3::ast::Amount,
) -> StepResult {
    let (domain_context, domain_errors) = ops.predicate(drs.clone(), domain);
    let kind = domain
        .kind()
        .or_else(|| drs.relative_kind.as_deref().cloned())
        .unwrap_or(Kind::Object);
    let mut candidate = domain_context.clone();
    candidate.next_clause();
    candidate.prepend(
        vec![domain.binding(Determiner::The, Plurality::One, &kind)],
        GramRole::Other,
        false,
    );
    let (measured, measure_errors) = ops.amount(candidate.clone(), measure);
    candidate.next_id = measured.next_id;
    let (mut bounded, bound_errors) = ops.amount(candidate.clone(), bound);
    let mut retained: Vec<_> = bounded
        .referents
        .iter()
        .filter(|r| r.id >= candidate.next_id)
        .cloned()
        .collect();
    retained.extend(
        domain_context
            .referents
            .iter()
            .filter(|r| r.id >= drs.next_id)
            .cloned(),
    );
    retained.extend(
        bounded
            .referents
            .iter()
            .filter(|r| r.id < drs.next_id)
            .cloned(),
    );
    bounded.referents = retained;
    bounded.current_clause = domain_context.current_clause;
    bounded.pending_roles = domain_context.pending_roles;
    bounded.prepend(
        vec![Referent::new(
            Determiner::The,
            Plurality::One,
            Payload::Difference,
        )],
        GramRole::Other,
        false,
    );
    let mut errors = refuse(kind.phrasal(), Refusal::Phrasal { k: Box::new(kind) });
    errors.extend(domain_errors);
    errors.extend(measure_errors);
    errors.extend(bound_errors);
    (bounded, errors)
}
fn step_attachment(
    ops: &impl StepOps,
    drs: DRS,
    side: &AttachmentSide,
    word: Option<&AttachWord>,
    counterpart: Option<&deckmaste_semantics_v3::ast::NounPhrase>,
) -> StepResult {
    let kind = head_kind(&drs);
    let mut errors = match side {
        AttachmentSide::Host => attachment_host_check(word, &kind),
        AttachmentSide::Attachment => kind_check(kind, Some(Kind::Object)),
    };
    let (out, more) = if let Some(counterpart) = counterpart {
        let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, counterpart);
        let kind = ids.first().and_then(|id| out.find_by_id(*id)).map_or_else(
            || counterpart.kind_in(&out, Kind::Object),
            |r| r.payload.kind(),
        );
        errors.extend(match side {
            AttachmentSide::Host => kind_check(Kind::Object, Some(kind)),
            AttachmentSide::Attachment => attachment_host_check(word, &kind),
        });
        (out, errors)
    } else {
        (drs, Vec::new())
    };
    errors.extend(more);
    (out, errors)
}
fn step_combat_predicate(
    ops: &impl StepOps,
    drs: DRS,
    relation: &CombatRelation,
    counterpart: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let head = head_kind(&drs);
    let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, counterpart);
    errors.extend(if *relation == CombatRelation::AttackedBy {
        refuse(
            head.lte(&either()),
            Refusal::KindLte {
                k: Box::new(head),
                bound: Box::new(either()),
            },
        )
    } else {
        kind_check(Kind::Object, Some(head))
    });
    if let Some(kind) = ids
        .first()
        .and_then(|id| out.find_by_id(*id))
        .map(|r| r.payload.kind())
    {
        errors.extend(
            if matches!(
                relation,
                CombatRelation::AttackedBy | CombatRelation::AttackerOf
            ) {
                refuse(
                    kind.lte(&either()),
                    Refusal::KindLte {
                        k: Box::new(kind),
                        bound: Box::new(either()),
                    },
                )
            } else {
                kind_check(Kind::Object, Some(kind))
            },
        );
    }
    (out, errors)
}
fn other_anchor_ok(ops: &impl StepOps, drs: &DRS, predicates: &[Predicate]) -> bool {
    let flattened: Vec<_> = predicates.iter().flat_map(Predicate::flattened).collect();
    if flattened
        .iter()
        .filter(|p| {
            matches!(
                p.scope_body(),
                Predicate::Other | Predicate::OtherThan { .. }
            )
        })
        .count()
        > 1
    {
        return false;
    }
    let head_types: Vec<_> = predicates
        .iter()
        .flat_map(Predicate::head_type_alternatives)
        .flatten()
        .collect();
    flattened.iter().all(|p| {
        if let Predicate::OtherThan { anchor } = p {
            let (out, ids, _) = ops.noun(drs.clone(), GramRole::Other, false, anchor);
            let anchor_types: Vec<_> = ids
                .iter()
                .filter_map(|id| out.find_by_id(*id))
                .flat_map(|r| r.payload.types())
                .collect();
            head_types.is_empty()
                || anchor_types.is_empty()
                || head_types.iter().any(|ty| anchor_types.contains(ty))
        } else {
            true
        }
    })
}
fn step_predicate_conjunction(
    ops: &impl StepOps,
    drs: &DRS,
    whole: &Predicate,
    items: &[Predicate],
) -> StepResult {
    let head_kind = whole
        .kind()
        .or_else(|| drs.relative_kind.as_deref().cloned());
    let kind = head_kind.clone().unwrap_or(Kind::Object);
    let seed = if kind == Kind::Object {
        whole.binding(Determiner::The, Plurality::One, &kind)
    } else {
        kind.binding()
    };
    let types: Vec<_> = items
        .iter()
        .flat_map(Predicate::head_type_alternatives)
        .flatten()
        .collect();
    let zone = whole.payload(&kind).zone().cloned();
    let mut out = drs.clone();
    let mut errors = Vec::new();
    for item in items {
        out.relative_kind = head_kind.clone().map(Box::new);
        out.relative_types.clone_from(&types);
        out.relative_zone.clone_from(&zone);
        out.relative_control = whole.control_fact();
        let (next, more) = if let Predicate::Did { clause } = item {
            super::step_relative_did_with(ops, &out, seed.clone(), clause)
        } else {
            ops.predicate(out, item)
        };
        out = next;
        errors.extend(more);
    }
    if let Some(head) = head_kind {
        for item in items {
            if let Some(kind) = item.kind() {
                errors.extend(refuse(
                    kind.lte(&head) || head.lte(&kind),
                    Refusal::KindMismatch {
                        expected: Box::new(head.clone()),
                        found: Box::new(kind),
                    },
                ));
            }
        }
    }
    out.relative_kind.clone_from(&drs.relative_kind);
    out.relative_types.clone_from(&drs.relative_types);
    out.relative_zone.clone_from(&drs.relative_zone);
    out.relative_control = drs.relative_control;
    errors.extend(refuse(
        other_anchor_ok(ops, drs, items),
        Refusal::OtherAnchored,
    ));
    (out, errors)
}
fn step_predicate_alternatives(
    ops: &impl StepOps,
    mut drs: DRS,
    whole: &Predicate,
    items: &[Predicate],
) -> StepResult {
    let head = whole
        .kind()
        .or_else(|| drs.relative_kind.as_deref().cloned());
    let mut errors = Vec::new();
    let mut kinds = Vec::new();
    for item in items {
        let mut branch = drs.clone();
        branch.relative_kind = item.kind().or_else(|| head.clone()).map(Box::new);
        let (out, more) = ops.predicate(branch, item);
        drs.next_id = out.next_id;
        errors.extend(more);
        if let Some(kind) = item.kind()
            && !kinds.contains(&kind)
        {
            kinds.push(kind);
        }
    }
    if kinds.len() <= 1 {
        errors.extend(refuse(!items.is_empty(), Refusal::NonEmpty));
        errors.extend(refuse(
            items.iter().all(|p| !p.has_other()),
            Refusal::CoordinableDisjuncts,
        ));
    }
    (drs, errors)
}

fn step_chosen(drs: DRS, ref_: &deckmaste_semantics_v3::ast::ChoiceRef) -> StepResult {
    let kind = head_kind(&drs);
    let errors = if let Some(sort) = ChoiceSort::at(&kind) {
        choice_errors(&drs, ref_, sort)
    } else {
        refuse(
            drs.count_chosen_objects(&kind) != 0,
            Refusal::ChoiceInScope { k: Box::new(kind) },
        )
    };
    (drs, errors)
}

fn step_quality_choice(
    ops: &impl StepOps,
    drs: DRS,
    sort: &deckmaste_semantics_v3::ast::QualitySort,
    domain: Option<&deckmaste_semantics_v3::ast::ChoiceDomain>,
) -> StepResult {
    let (out, more) = super::sorted_domain_check_with(
        ops,
        drs,
        &ChoiceSort::Quality { sort: sort.clone() },
        domain,
    );
    let mut errors = refuse(
        sort.chosen_read_ok(),
        Refusal::ChosenQualityRead { q: sort.clone() },
    );
    errors.extend(more);
    (out, errors)
}

fn step_each_kind(
    ops: &impl StepOps,
    drs: DRS,
    domain: Option<&deckmaste_semantics_v3::ast::NounPhrase>,
) -> StepResult {
    let placement = drs.each_kind_quality;
    let (out, mut errors) = if let Some(domain) = domain {
        let (out, _, errors) = ops.noun(drs, GramRole::Other, false, domain);
        (out, errors)
    } else {
        (drs, Vec::new())
    };
    errors.extend(refuse(placement, Refusal::EachKindPlacement));
    (out, errors)
}

fn step_chosen_quality(
    drs: DRS,
    ref_: &deckmaste_semantics_v3::ast::ChoiceRef,
    sort: &deckmaste_semantics_v3::ast::QualitySort,
) -> StepResult {
    let mut errors = refuse(
        sort.chosen_read_ok(),
        Refusal::ChosenQualityRead { q: sort.clone() },
    );
    errors.extend(choice_errors(
        &drs,
        ref_,
        ChoiceSort::Quality { sort: sort.clone() },
    ));
    (drs, errors)
}

fn step_compare(
    ops: &impl StepOps,
    drs: DRS,
    axes: &[deckmaste_semantics_v3::ast::ProjAxis],
    bound: &deckmaste_semantics_v3::ast::Amount,
) -> StepResult {
    let kind = head_kind(&drs);
    let (out, more) = ops.amount(drs, bound);
    let mut errors = refuse(
        !axes.is_empty() && axes.iter().all(|axis| axis.scope() == kind),
        Refusal::AxesAt { k: Box::new(kind) },
    );
    errors.extend(more);
    (out, errors)
}

fn step_superlative(
    ops: &impl StepOps,
    drs: DRS,
    op: &deckmaste_semantics_v3::ast::AggregateOp,
    axis: &deckmaste_semantics_v3::ast::ProjAxis,
    domain: &deckmaste_semantics_v3::ast::Predicate,
) -> StepResult {
    let kind = head_kind(&drs);
    let (out, more) = ops.predicate(drs, domain);
    let mut errors = refuse(
        matches!(op, AggregateOp::Min | AggregateOp::Max),
        Refusal::IsExtremal,
    );
    errors.extend(refuse(
        axis.scope() == kind,
        Refusal::ProjScope { k: Box::new(kind) },
    ));
    errors.extend(more);
    (out, errors)
}

fn step_chose_extreme(mut drs: DRS) -> StepResult {
    let count = drs.count_many(&Kind::Quality {
        sort: QualitySort::Number,
    });
    drs.prepend(
        vec![Referent::new(
            Determiner::The,
            Plurality::One,
            Payload::Outcome {
                sort: OutcomeSort::NamedNumber,
            },
        )],
        GramRole::Other,
        false,
    );
    (drs, refuse(count != 0, Refusal::NumberChoiceInScope))
}

fn step_designation(
    ops: &impl StepOps,
    drs: DRS,
    designation: &deckmaste_semantics_v3::ast::DesignationLabel,
    holder: Option<&deckmaste_semantics_v3::ast::NounPhrase>,
) -> StepResult {
    let kind = head_kind(&drs);
    let (out, more) = if let Some(owner) = holder {
        let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, owner);
        errors.extend(kind_check(
            Kind::Player,
            ids.first()
                .and_then(|id| out.find_by_id(*id))
                .map(|r| r.payload.kind()),
        ));
        (out, errors)
    } else {
        (drs, Vec::new())
    };
    let mut errors = refuse(
        designation.holder() == Some(kind.clone()),
        Refusal::DesignationHolder {
            designation: designation.clone(),
            k: Box::new(kind),
        },
    );
    errors.extend(refuse(
        holder.is_none() || designation.holder() == Some(Kind::Object),
        Refusal::DesignationPossessorFits {
            designation: designation.clone(),
        },
    ));
    errors.extend(more);
    (out, errors)
}

fn step_other(mut drs: DRS) -> StepResult {
    let kind = head_kind(&drs);
    if let Some(anchor) = drs.contrast_anchor(
        &kind,
        &drs.relative_types,
        drs.relative_zone.as_ref(),
        drs.relative_control,
    ) {
        drs.contrast_anchors.insert(0, anchor);
        (drs, Vec::new())
    } else {
        (drs, vec![Refusal::ContrastAnchor { k: Box::new(kind) }])
    }
}

fn step_targets(
    ops: &impl StepOps,
    drs: DRS,
    subject: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let head = head_kind(&drs);
    let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, subject);
    let target = ids
        .first()
        .and_then(|id| out.find_by_id(*id))
        .map_or(Kind::Object, |r| r.payload.kind());
    errors.extend(refuse(
        target.targetable(),
        Refusal::Targetable {
            k: Box::new(target),
        },
    ));
    errors.extend(refuse(
        head.targeter(),
        Refusal::Targeter { k: Box::new(head) },
    ));
    (out, errors)
}

fn step_coin_outcome(drs: DRS) -> StepResult {
    let kind = head_kind(&drs);
    let mut errors = refuse(
        drs.count_outcomes(&OutcomeSort::CoinFlipped) == 1,
        Refusal::CoinFlipInScope,
    );
    errors.extend(refuse(
        kind.lte(&either()),
        Refusal::KindLte {
            k: Box::new(kind),
            bound: Box::new(either()),
        },
    ));
    (drs, errors)
}

fn step_most_votes(drs: DRS) -> StepResult {
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

fn step_player_relation(
    ops: &impl StepOps,
    drs: DRS,
    who: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let (out, ids, mut errors) = ops.noun(drs.clone(), GramRole::Other, false, who);
    errors.extend(kind_check(
        Kind::Player,
        ids.first()
            .and_then(|id| out.find_by_id(*id))
            .map(|r| r.payload.kind()),
    ));
    (if who.is_you() { drs } else { out }, errors)
}

fn step_shares(
    ops: &impl StepOps,
    drs: DRS,
    with_: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, with_);
    errors.extend(kind_check(
        Kind::Object,
        ids.first()
            .and_then(|id| out.find_by_id(*id))
            .map(|r| r.payload.kind()),
    ));
    (out, errors)
}

fn step_chosen_name(drs: DRS) -> StepResult {
    let errors = choice_errors(
        &drs,
        &ChoiceRef::TheChoice,
        ChoiceSort::Quality {
            sort: QualitySort::CardName,
        },
    );
    (drs, errors)
}

fn step_counter_holder(drs: DRS) -> StepResult {
    let kind = head_kind(&drs);
    (
        drs,
        refuse(
            matches!(kind, Kind::Object | Kind::Player),
            Refusal::CounterHolderKind,
        ),
    )
}
