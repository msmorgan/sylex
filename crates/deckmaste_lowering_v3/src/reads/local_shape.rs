//! Local shape checks from `Semantics/Check/LocalShape.lean`.
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::ArithOp;
use deckmaste_semantics_v3::ast::ChoiceRef;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::ast::CostShift;
use deckmaste_semantics_v3::ast::CounterKindSource;
use deckmaste_semantics_v3::ast::CreationSpec;
use deckmaste_semantics_v3::ast::DieSides;
use deckmaste_semantics_v3::ast::ManaCostTerm;
use deckmaste_semantics_v3::ast::ManaHeld;
use deckmaste_semantics_v3::ast::ManaRider;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::ProducedMana;
use deckmaste_semantics_v3::ast::QualitySort;
use deckmaste_semantics_v3::ast::Quantity;
use deckmaste_semantics_v3::ast::RollWatch;

use super::DRS;
use crate::reads::AmountSelectionsExt;
use crate::reads::DeltaWordsExt;
use crate::refusal::Refusal;

fn refuse(ok: bool, error: Refusal) -> Vec<Refusal> {
    if ok { vec![] } else { vec![error] }
}
fn outcome(drs: &DRS, sort: OutcomeSort) -> Vec<Refusal> {
    let found = drs.count_outcomes(&sort) as u64;
    refuse(found == 1, Refusal::OutcomeInScope { s: sort, found })
}
fn choice(drs: &DRS, quality: QualitySort) -> Vec<Refusal> {
    let sort = ChoiceSort::Quality { sort: quality };
    let found = drs.count_choice(&sort) as u64;
    refuse(
        found == 1,
        Refusal::ChoiceRef {
            ref_: ChoiceRef::TheChoice,
            sort,
            found,
        },
    )
}
/// Discourse inspection of `Amount`.
pub trait AmountLocalShapeExt: Sized {
    #[must_use]
    fn non_zero(&self) -> bool;
    #[doc(hidden)]
    fn non_zero_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn for_each(&self) -> bool;
    #[must_use]
    fn unit(&self) -> bool;
}

impl AmountLocalShapeExt for Amount {
    fn non_zero(&self) -> bool {
        self.non_zero_at(64)
    }
    fn non_zero_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.non_zero_at(fuel - 1),
            (1.., Self::Lit { value }) => *value != 0,
            _ => true,
        }
    }
    fn for_each(&self) -> bool {
        let mut amount = self;
        for _ in 0..64 {
            match amount {
                Self::InCaller { body } => amount = body,
                Self::Arith {
                    op: ArithOp::Times, ..
                } => return true,
                _ => return false,
            }
        }
        false
    }
    fn unit(&self) -> bool {
        self.exact() == Some(1)
    }
}
/// Discourse inspection of `Quantity`.
pub trait QuantityLocalShapeExt: Sized {
    #[must_use]
    fn non_zero(&self) -> bool;
    #[doc(hidden)]
    fn non_zero_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn literal(&self) -> bool;
    #[must_use]
    fn modes_fit(&self, count: u64) -> bool;
}

impl QuantityLocalShapeExt for Quantity {
    fn non_zero(&self) -> bool {
        self.non_zero_at(64)
    }
    fn non_zero_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.non_zero_at(fuel - 1),
            (1.., Self::Range { high: Some(0), .. } | Self::Worth { n: 0 }) => false,
            (1.., Self::ExactlyOf { amount } | Self::UpToOf { amount }) => {
                amount.non_zero_at(fuel - 1)
            }
            _ => true,
        }
    }
    fn literal(&self) -> bool {
        let mut quantity = self;
        for _ in 0..64 {
            match quantity {
                Self::InCaller { body } => quantity = body,
                Self::Range { .. } => return true,
                _ => return false,
            }
        }
        false
    }
    fn modes_fit(&self, count: u64) -> bool {
        let mut quantity = self;
        for _ in 0..64 {
            match quantity {
                Self::InCaller { body } => quantity = body,
                Self::Range {
                    high: Some(upper), ..
                } => return *upper <= count,
                Self::Range {
                    low: Some(lower),
                    high: None,
                } => return *lower <= count,
                _ => return true,
            }
        }
        true
    }
}
/// Discourse inspection of `RollWatch`.
pub trait RollWatchLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self) -> Vec<Refusal>;
}

impl RollWatchLocalShapeExt for RollWatch {
    fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::ResultIn { quantity } => {
                let mut errors = refuse(quantity.non_zero(), Refusal::NonZeroQ);
                errors.extend(refuse(quantity.literal(), Refusal::QuantLiteral));
                errors
            }
            Self::AnyResult | Self::HighestNatural => vec![],
        }
    }
}
/// Discourse inspection of `DieSides`.
pub trait DieSidesLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal>;
}

impl DieSidesLocalShapeExt for DieSides {
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::Sides { sides } => refuse(*sides != 0, Refusal::NonZeroQ),
            Self::ThoseDice => outcome(drs, OutcomeSort::DiceRolled),
            Self::AnyDie => vec![],
        }
    }
}
/// Discourse inspection of `ProducedMana`.
pub trait ProducedManaLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal>;
}

impl ProducedManaLocalShapeExt for ProducedMana {
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::Runs { runs } => refuse(
                !runs.is_empty() && runs.iter().all(|run| !run.is_empty()),
                Refusal::ProducedRuns,
            ),
            Self::OfChosenColor { .. } => choice(drs, QualitySort::Color),
            Self::ProducedByEvent { .. } => outcome(drs, OutcomeSort::ManaProduced),
            _ => vec![],
        }
    }
}
/// Discourse inspection of `ManaRider`.
pub trait ManaRiderLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self) -> Vec<Refusal>;
}

impl ManaRiderLocalShapeExt for ManaRider {
    fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::SpendOnly { purposes } | Self::SpendNotOn { purposes } => {
                refuse(!purposes.is_empty(), Refusal::NonEmpty)
            }
            Self::OnSpent { .. } => vec![],
        }
    }
}
/// Discourse inspection of `ManaHeld`.
pub trait ManaHeldLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal>;
}

impl ManaHeldLocalShapeExt for ManaHeld {
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::ThisMana => outcome(drs, OutcomeSort::ManaAdded),
            Self::Unspent { .. } => vec![],
        }
    }
}
/// Discourse inspection of `Cost`.
pub trait CostLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self) -> Vec<Refusal>;
}

impl CostLocalShapeExt for Cost {
    fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::Mana { cost } => refuse(!cost.is_empty(), Refusal::NonEmpty),
            Self::Scaled { amount, .. } => refuse(amount.for_each(), Refusal::ForEachAmount),
            _ => vec![],
        }
    }
}
/// Discourse inspection of `CostShift`.
pub trait CostShiftLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self) -> Vec<Refusal>;
}

impl CostShiftLocalShapeExt for CostShift {
    fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::Generic { change, .. } => refuse(!change.is_set(), Refusal::ShiftSets),
            Self::BySymbols { change, .. } => {
                let mut errors = refuse(!change.is_set(), Refusal::ShiftSets);
                if let ManaCostTerm::Lit { cost } = change.amount().as_ref() {
                    errors.extend(refuse(!cost.is_empty(), Refusal::NonEmpty));
                }
                errors
            }
        }
    }
}
/// Discourse inspection of `CreationSpec`.
pub trait CreationSpecLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self) -> Vec<Refusal>;
}

impl CreationSpecLocalShapeExt for CreationSpec {
    fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::Emblem { abilities, .. } => refuse(!abilities.is_empty(), Refusal::NonEmpty),
            Self::Token { .. } => vec![],
        }
    }
}
/// Discourse inspection of `CounterKindSource`.
pub trait CounterKindSourceLocalShapeExt: Sized {
    #[must_use]
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal>;
}

impl CounterKindSourceLocalShapeExt for CounterKindSource {
    fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::Chosen { menu } | Self::DistinctChosen { menu } => {
                refuse(!menu.is_empty(), Refusal::NonEmpty)
            }
            Self::Bound => choice(drs, QualitySort::CounterKind),
            Self::Those => {
                let put = drs.count_outcomes(&OutcomeSort::CountersPut) as u64;
                let removed = drs.count_outcomes(&OutcomeSort::CountersRemoved) as u64;
                if put + removed == 1 {
                    vec![]
                } else if removed == 0 {
                    vec![Refusal::OutcomeInScope {
                        s: OutcomeSort::CountersPut,
                        found: put,
                    }]
                } else {
                    vec![Refusal::OutcomeInScope {
                        s: OutcomeSort::CountersRemoved,
                        found: put + removed,
                    }]
                }
            }
            Self::Printed { .. } | Self::SameAs { .. } | Self::AnyKind => vec![],
        }
    }
}
