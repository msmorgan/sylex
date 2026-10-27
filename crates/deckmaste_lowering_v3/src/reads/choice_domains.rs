//! Choice domains and their sorted reads.
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::ChoiceDomain;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::ColorTerm;
use deckmaste_semantics_v3::ast::KeywordLabel;
use deckmaste_semantics_v3::ast::KeywordTerm;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::QualitySort;

use super::DRS;
use super::GramRole;
use super::StepOps;
use super::StepResult;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::reads::CardTypeWordsExt;
use crate::reads::ChoiceRefWordsExt;
use crate::reads::PredicateFactsExt;
use crate::refusal::Refusal;
/// Discourse inspection of `KeywordLabel`.
pub trait KeywordLabelChoiceDomainsExt: Sized {
    #[must_use]
    fn names_keyword(&self) -> bool;
}

impl KeywordLabelChoiceDomainsExt for KeywordLabel {
    fn names_keyword(&self) -> bool {
        let Self::Named { name } = self;
        !name.as_str().is_empty()
    }
}
/// Discourse inspection of `KeywordTerm`.
pub trait KeywordTermChoiceDomainsExt: Sized {
    #[must_use]
    fn known(&self) -> bool;
}

impl KeywordTermChoiceDomainsExt for KeywordTerm {
    fn known(&self) -> bool {
        match self {
            Self::The { keyword } | Self::TheWith { keyword, .. } => keyword.names_keyword(),
            Self::AnyIn { family } => family.word.names_keyword(),
        }
    }
}
/// Discourse inspection of `ChoiceDomain`.
pub trait ChoiceDomainExt: Sized {
    #[must_use]
    fn sort(&self) -> ChoiceSort;
}

impl ChoiceDomainExt for ChoiceDomain {
    fn sort(&self) -> ChoiceSort {
        match self {
            Self::NameOfCard { .. } => ChoiceSort::Quality {
                sort: QualitySort::CardName,
            },
            Self::ColorOtherThan { .. } => ChoiceSort::Quality {
                sort: QualitySort::Color,
            },
            Self::TypeOtherThan { .. } => ChoiceSort::Quality {
                sort: QualitySort::Subtype {
                    host: CardType::creature(),
                },
            },
            Self::BasicTypesOnly | Self::NonbasicTypesOnly => ChoiceSort::Quality {
                sort: QualitySort::Subtype {
                    host: CardType::land(),
                },
            },
            Self::Number { .. } => ChoiceSort::Quality {
                sort: QualitySort::Number,
            },
            Self::Players { .. } => ChoiceSort::Player,
            Self::AbilitiesAmong { .. } => ChoiceSort::Quality {
                sort: QualitySort::Ability,
            },
        }
    }
}
pub fn step_choice_domain_with(ops: &impl StepOps, drs: DRS, domain: &ChoiceDomain) -> StepResult {
    match domain {
        ChoiceDomain::NameOfCard { predicate } => {
            let mut context = drs.clone();
            context.enter_own_scope(None);
            let (_, mut errors) = ops.predicate(context, predicate);
            errors.extend(kind_check(Kind::Object, predicate.kind()));
            (drs, errors)
        }
        ChoiceDomain::Players { predicate } => {
            let (out, mut errors) = ops.predicate(drs, predicate);
            errors.extend(kind_check(Kind::Player, predicate.kind()));
            (out, errors)
        }
        ChoiceDomain::Number { quantity } => super::step_quantity_with(ops, drs, quantity),
        ChoiceDomain::AbilitiesAmong { keywords } => {
            let mut errors = refuse(!keywords.is_empty(), Refusal::NonEmpty);
            errors.extend(refuse(
                keywords.iter().all(KeywordTerm::known),
                Refusal::KnownKeywordTerm,
            ));
            (drs, errors)
        }
        ChoiceDomain::TypeOtherThan { .. }
        | ChoiceDomain::ColorOtherThan { .. }
        | ChoiceDomain::BasicTypesOnly
        | ChoiceDomain::NonbasicTypesOnly => (drs, Vec::new()),
    }
}
pub fn sorted_domain_check_with(
    ops: &impl StepOps,
    drs: DRS,
    sort: &ChoiceSort,
    domain: Option<&ChoiceDomain>,
) -> StepResult {
    if let Some(domain) = domain {
        let (out, mut errors) = step_choice_domain_with(ops, drs, domain);
        errors.extend(refuse(domain.sort() == *sort, Refusal::KindAxisSort));
        (out, errors)
    } else {
        (drs, Vec::new())
    }
}
pub fn step_color_term_with(ops: &impl StepOps, drs: DRS, term: &ColorTerm) -> StepResult {
    match term {
        ColorTerm::Lit { .. } => (drs, Vec::new()),
        ColorTerm::Chosen { ref_ } => {
            let sort = ChoiceSort::Quality {
                sort: QualitySort::Color,
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
        ColorTerm::ColorOf { object } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Other, false, object);
            (out, errors)
        }
    }
}
