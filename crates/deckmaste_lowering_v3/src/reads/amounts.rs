//! Amount stepping and measure-local discourse boxes.
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::ArithOp;
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::KindAxis;
use deckmaste_semantics_v3::ast::LinkSource;
use deckmaste_semantics_v3::ast::LinkedFacet;
use deckmaste_semantics_v3::ast::Ordinal;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::ProjAxis;
use deckmaste_semantics_v3::ast::QualitySort;
use deckmaste_semantics_v3::ast::SubtypeScope;

use super::DRS;
use super::Determiner;
use super::GramRole;
use super::Payload;
use super::Referent;
use super::StepOps;
use super::StepResult;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::reads::AmountLocalShapeExt;
use crate::reads::CardTypeWordsExt;
use crate::reads::ChoiceRefWordsExt;
use crate::reads::KindResolutionExt;
use crate::reads::NounPhraseFactsExt;
use crate::reads::NounPhrasePhraseFactsExt;
use crate::reads::PredicateFactsExt;
use crate::reads::ProjAxisPredicateFactsExt;
use crate::reads::TallyModeEventsExt;
use crate::refusal::Refusal;
/// Discourse inspection of `KindAxis`.
pub trait KindAxisAmountsExt: Sized {
    #[must_use]
    fn valid_sort(&self) -> bool;
}

impl KindAxisAmountsExt for KindAxis {
    fn valid_sort(&self) -> bool {
        match self {
            Self::Subtype { host, scope } => {
                *scope == SubtypeScope::Any || *host == CardType::land()
            }
            _ => true,
        }
    }
}
/// Discourse inspection of `LinkedFacet`.
pub trait LinkedFacetAmountsExt: Sized {
    #[must_use]
    fn named(&self) -> bool;
}

impl LinkedFacetAmountsExt for LinkedFacet {
    fn named(&self) -> bool {
        match self {
            Self::ColorsSpent | Self::ManaValueSpent | Self::PhyrexianPaidWithLife => true,
            Self::TimesPaid { which } | Self::Readback { which, .. } | Self::Taken { which } => {
                !matches!(
                    which,
                    LinkSource::ByNthKeyword {
                        ordinal: Ordinal::Nth { n: 0 },
                        ..
                    }
                )
            }
        }
    }
}
fn outcome_errors(drs: &DRS, sort: OutcomeSort) -> Vec<Refusal> {
    let actual = drs.count_outcomes(&sort);
    let found = if sort == OutcomeSort::RollResult && actual == 0 {
        drs.count_outcomes(&OutcomeSort::DiceRolled)
    } else {
        actual
    };
    refuse(
        found == 1,
        Refusal::OutcomeInScope {
            s: sort,
            found: found as u64,
        },
    )
}
pub fn step_amount_with(ops: &impl StepOps, drs: DRS, amount: &Amount) -> StepResult {
    match amount {
        Amount::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, errors) = ops.amount(caller, body);
            out.leave_caller(&drs, &[]);
            (out, errors)
        }
        Amount::Lit { .. } | Amount::AnyAmount | Amount::TheNth { .. } => (drs, Vec::new()),
        Amount::StatOf { axis, subject } => step_stat(ops, drs, axis, subject),
        Amount::Paid { facet, subject } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, subject);
            errors.extend(refuse(facet.named(), Refusal::LinkedFacetNamed));
            (out, errors)
        }
        Amount::Aggregate { axis, group, .. } => step_aggregate(ops, &drs, axis, group),
        Amount::CountOf { group } => {
            let (out, _, mut errors) = ops.noun(drs.clone(), GramRole::Other, false, group);
            errors.extend(refuse(
                group.plurality() == Plurality::Many,
                Refusal::Plural,
            ));
            errors.extend(refuse(group.countable_group(), Refusal::CountableGroup));
            (DRS::leave_measure(&drs, &out), errors)
        }
        Amount::GreatestStoredMatch { subject } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Other, false, subject);
            (out, errors)
        }
        Amount::DistinctCount { axis, domain } => {
            let (out, _, mut errors) = ops.noun(drs, GramRole::Other, false, domain);
            errors.extend(refuse(axis.valid_sort(), Refusal::KindAxisSort));
            (out, errors)
        }
        Amount::Tally { clause, mode } => {
            let (_, mut errors) = super::step_did_with(ops, &drs, clause);
            errors.extend(refuse(mode.permits(clause), Refusal::TallyOk));
            errors.extend(refuse(clause.within.is_some(), Refusal::UnanchoredLookback));
            (drs, errors)
        }
        Amount::ThatMuch => {
            let found = drs.count_amount_outcomes();
            (
                drs,
                refuse(
                    found == 1,
                    Refusal::QuantOutcomeInScope {
                        found: found as u64,
                    },
                ),
            )
        }
        Amount::ChosenNumber { ref_ } => {
            let sort = ChoiceSort::Quality {
                sort: QualitySort::Number,
            };
            let found = drs.count_choice(&sort);
            (
                drs,
                refuse(
                    ref_.permits(found),
                    Refusal::ChoiceRef {
                        ref_: ref_.clone(),
                        sort,
                        found: found as u64,
                    },
                ),
            )
        }
        Amount::VotesFor { .. } => {
            let errors = outcome_errors(&drs, OutcomeSort::VoteHeld);
            (drs, errors)
        }
        Amount::TheOutcome { sort } => {
            let errors = outcome_errors(&drs, sort.clone());
            (drs, errors)
        }
        Amount::CoinsShowing { .. } => {
            let ok = drs.count_outcomes(&OutcomeSort::CoinFlipped) == 1;
            (drs, refuse(ok, Refusal::CoinFlipInScope))
        }
        Amount::GroupSize => step_group_size(drs),
        Amount::TheDifference => {
            let found = drs.count_differences();
            (
                drs,
                refuse(
                    found == 1,
                    Refusal::DifferenceInScope {
                        found: found as u64,
                    },
                ),
            )
        }
        Amount::Letter { letter } => step_letter(drs, letter),
        Amount::Arith { op, left, right } => {
            let (out, mut errors) = ops.amount(drs, left);
            let (out, more) = ops.amount(out, right);
            errors.extend(more);
            errors.extend(refuse(
                *op != ArithOp::Times || left.non_zero(),
                Refusal::AmtNonZero,
            ));
            (out, errors)
        }
        Amount::Devotion {
            player,
            color,
            second,
        } => step_devotion(ops, drs, player, color, second.as_deref()),
        Amount::Half { amount, .. } => ops.amount(drs, amount),
        Amount::AggregateOver { domain, body, .. } => {
            let (mut out, mut errors) = ops.predicate(drs, domain);
            let kind = domain.kind().unwrap_or(Kind::Object);
            out.prepend(
                vec![domain.binding(Determiner::The, Plurality::One, &kind)],
                GramRole::Other,
                false,
            );
            let (out, more) = ops.amount(out, body);
            errors.extend(more);
            (out, errors)
        }
        Amount::AggregateOf { amounts, .. } => {
            amounts
                .iter()
                .fold((drs, Vec::new()), |(d, mut errors), amount| {
                    let (out, more) = ops.amount(d, amount);
                    errors.extend(more);
                    (out, errors)
                })
        }
    }
}
fn step_stat(
    ops: &impl StepOps,
    mut drs: DRS,
    axis: &ProjAxis,
    subject: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let characteristic = drs.characteristic_read;
    drs.characteristic_read = true;
    let mut errors = if *axis == ProjAxis::BoundCounter {
        let sort = ChoiceSort::Quality {
            sort: QualitySort::CounterKind,
        };
        let found = drs.count_choice(&sort);
        refuse(
            found == 1,
            Refusal::ChoiceRef {
                ref_: deckmaste_semantics_v3::ast::ChoiceRef::TheChoice,
                sort,
                found: found as u64,
            },
        )
    } else {
        Vec::new()
    };
    let (mut out, ids, more) = ops.noun(drs, GramRole::Other, false, subject);
    errors.extend(more);
    let actual = subject.selection_kind_override(64).or_else(|| {
        ids.first()
            .and_then(|id| out.find_by_id(*id))
            .map(|r| r.payload.kind())
    });
    let plural = subject.determiner() == Some(Determiner::Each)
        || subject.plurality() != Plurality::One
        || ids
            .iter()
            .filter_map(|id| out.find_by_id(*id))
            .any(|r| r.plur != Plurality::One);
    if *axis == ProjAxis::BoundCounter {
        if let Some(kind) = actual {
            errors.extend(refuse(
                matches!(kind, Kind::Object | Kind::Player),
                Refusal::CounterHolderKind,
            ));
        }
    } else {
        errors.extend(kind_check(axis.scope(), actual));
    }
    errors.extend(refuse(!plural, Refusal::Singular));
    out.characteristic_read = characteristic;
    (out, errors)
}
fn step_aggregate(
    ops: &impl StepOps,
    drs: &DRS,
    axis: &ProjAxis,
    subject: &deckmaste_semantics_v3::ast::NounPhrase,
) -> StepResult {
    let (out, ids, mut errors) = ops.noun(drs.clone(), GramRole::Other, false, subject);
    let kind = ids
        .first()
        .and_then(|id| out.find_by_id(*id))
        .map_or_else(|| subject.kind_in(&out, Kind::Object), |r| r.payload.kind());
    let plural = subject.determiner() == Some(Determiner::Each)
        || subject.plurality() == Plurality::Many
        || ids
            .iter()
            .filter_map(|id| out.find_by_id(*id))
            .any(|r| r.plur == Plurality::Many);
    errors.extend(refuse(
        axis.scope() == kind,
        Refusal::ProjScope { k: Box::new(kind) },
    ));
    errors.extend(refuse(plural || axis.multi_valued(), Refusal::Plural));
    (DRS::leave_measure(drs, &out), errors)
}

fn step_group_size(drs: DRS) -> StepResult {
    let found = if let Some(ids) = &drs.group_size_scope {
        let mut ids = ids.clone();
        ids.sort_unstable();
        ids.dedup();
        super::whole_groups(
            &ids.iter()
                .filter_map(|id| drs.find_by_id(*id))
                .filter(|r| r.plur != Plurality::One)
                .collect::<Vec<_>>(),
        )
        .len()
    } else {
        drs.count_many_any()
    };
    (
        drs,
        refuse(
            found == 1,
            Refusal::GroupSizeInScope {
                found: found as u64,
            },
        ),
    )
}

fn step_letter(mut drs: DRS, letter: &deckmaste_semantics_v3::ast::Letter) -> StepResult {
    let kind = Kind::Letter {
        letter: letter.clone(),
    };
    if !drs
        .visible_referents()
        .any(|r| r.plur == Plurality::One && kind.lte(&r.payload.kind()))
    {
        drs.prepend(
            vec![Referent::new(
                Determiner::A,
                Plurality::One,
                Payload::Letter { l: letter.clone() },
            )],
            GramRole::Other,
            false,
        );
    }
    (drs, Vec::new())
}

fn step_devotion(
    ops: &impl StepOps,
    drs: DRS,
    player: &deckmaste_semantics_v3::ast::NounPhrase,
    color: &deckmaste_semantics_v3::ast::ColorTerm,
    second: Option<&deckmaste_semantics_v3::ast::ColorTerm>,
) -> StepResult {
    let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, player);
    errors.extend(kind_check(
        Kind::Player,
        ids.first()
            .and_then(|id| out.find_by_id(*id))
            .map(|r| r.payload.kind()),
    ));
    errors.extend(refuse(
        player.plurality() == Plurality::One
            && ids
                .iter()
                .filter_map(|id| out.find_by_id(*id))
                .all(|r| r.plur == Plurality::One),
        Refusal::Singular,
    ));
    let (out, more) = super::step_color_term_with(ops, out, color);
    errors.extend(more);
    let (out, more) = if let Some(second) = second {
        super::step_color_term_with(ops, out, second)
    } else {
        (out, Vec::new())
    };
    errors.extend(more);
    (out, errors)
}
