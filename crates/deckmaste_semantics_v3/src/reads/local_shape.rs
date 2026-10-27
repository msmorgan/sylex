//! Local shape checks from `Semantics/Check/LocalShape.lean`.
use super::DRS;
use crate::ast::Amount;
use crate::ast::ArithOp;
use crate::ast::ChoiceRef;
use crate::ast::ChoiceSort;
use crate::ast::Cost;
use crate::ast::CostShift;
use crate::ast::CounterKindSource;
use crate::ast::CreationSpec;
use crate::ast::DieSides;
use crate::ast::ManaCostTerm;
use crate::ast::ManaHeld;
use crate::ast::ManaRider;
use crate::ast::OutcomeSort;
use crate::ast::ProducedMana;
use crate::ast::QualitySort;
use crate::ast::Quantity;
use crate::ast::RollWatch;
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
impl Amount {
    #[must_use]
    pub fn non_zero(&self) -> bool {
        self.non_zero_at(64)
    }
    fn non_zero_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.non_zero_at(fuel - 1),
            (1.., Self::Lit { value }) => *value != 0,
            _ => true,
        }
    }
    #[must_use]
    pub fn for_each(&self) -> bool {
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
    #[must_use]
    pub fn unit(&self) -> bool {
        self.exact() == Some(1)
    }
}
impl Quantity {
    #[must_use]
    pub fn non_zero(&self) -> bool {
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
    #[must_use]
    pub fn literal(&self) -> bool {
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
    #[must_use]
    pub fn modes_fit(&self, count: u64) -> bool {
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
impl RollWatch {
    #[must_use]
    pub fn local_checks(&self) -> Vec<Refusal> {
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
impl DieSides {
    #[must_use]
    pub fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::Sides { sides } => refuse(*sides != 0, Refusal::NonZeroQ),
            Self::ThoseDice => outcome(drs, OutcomeSort::DiceRolled),
            Self::AnyDie => vec![],
        }
    }
}
impl ProducedMana {
    #[must_use]
    pub fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
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
impl ManaRider {
    #[must_use]
    pub fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::SpendOnly { purposes } | Self::SpendNotOn { purposes } => {
                refuse(!purposes.is_empty(), Refusal::NonEmpty)
            }
            Self::OnSpent { .. } => vec![],
        }
    }
}
impl ManaHeld {
    #[must_use]
    pub fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
        match self {
            Self::ThisMana => outcome(drs, OutcomeSort::ManaAdded),
            Self::Unspent { .. } => vec![],
        }
    }
}
impl Cost {
    #[must_use]
    pub fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::Mana { cost } => refuse(!cost.is_empty(), Refusal::NonEmpty),
            Self::Scaled { amount, .. } => refuse(amount.for_each(), Refusal::ForEachAmount),
            _ => vec![],
        }
    }
}
impl CostShift {
    #[must_use]
    pub fn local_checks(&self) -> Vec<Refusal> {
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
impl CreationSpec {
    #[must_use]
    pub fn local_checks(&self) -> Vec<Refusal> {
        match self {
            Self::Emblem { abilities, .. } => refuse(!abilities.is_empty(), Refusal::NonEmpty),
            Self::Token { .. } => vec![],
        }
    }
}
impl CounterKindSource {
    #[must_use]
    pub fn local_checks(&self, drs: &DRS) -> Vec<Refusal> {
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
