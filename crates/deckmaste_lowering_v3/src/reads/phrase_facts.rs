//! Determiners and plurality from `Semantics/Check/Phrase.lean`.
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::DetPhrase;
use deckmaste_semantics_v3::ast::Location;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Quantity;
use deckmaste_semantics_v3::ast::SliceCount;

use super::Determiner;

/// Discourse inspection of `Amount`.
pub trait AmountPhraseFactsExt: Sized {
    #[must_use]
    fn plurality(&self) -> Plurality;
    #[doc(hidden)]
    fn plurality_at(&self, fuel: u64) -> Plurality;
}

impl AmountPhraseFactsExt for Amount {
    fn plurality(&self) -> Plurality {
        self.plurality_at(64)
    }
    fn plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.plurality_at(fuel - 1),
            (1.., Self::Lit { value: 1 } | Self::TheNth { .. }) => Plurality::One,
            _ => Plurality::Many,
        }
    }
}
/// Discourse inspection of `Quantity`.
pub trait QuantityPhraseFactsExt: Sized {
    #[must_use]
    fn plurality(&self) -> Plurality;
    #[doc(hidden)]
    fn plurality_at(&self, fuel: u64) -> Plurality;
}

impl QuantityPhraseFactsExt for Quantity {
    fn plurality(&self) -> Plurality {
        self.plurality_at(64)
    }
    fn plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (0, _) => Plurality::Many,
            (_, Self::InCaller { body } | Self::Additional { quantity: body }) => {
                body.plurality_at(fuel - 1)
            }
            (_, Self::Range { high: Some(1), .. } | Self::TheLast) => Plurality::One,
            (_, Self::ExactlyOf { amount } | Self::UpToOf { amount }) => {
                amount.plurality_at(fuel - 1)
            }
            _ => Plurality::Many,
        }
    }
}
/// Discourse inspection of `DetPhrase`.
pub trait DetPhrasePhraseFactsExt: Sized {
    #[must_use]
    fn determiner(&self) -> Determiner;
    #[must_use]
    fn plurality(&self) -> Plurality;
}

impl DetPhrasePhraseFactsExt for DetPhrase {
    fn determiner(&self) -> Determiner {
        match self {
            Self::Target { .. } => Determiner::Target,
            Self::A { .. } => Determiner::A,
            Self::Each => Determiner::Each,
            Self::All => Determiner::All,
            Self::The { .. } => Determiner::The,
            Self::Count { .. } => Determiner::Count,
        }
    }
    fn plurality(&self) -> Plurality {
        match self {
            Self::Target { quantity } | Self::Count { quantity, .. } => quantity.plurality(),
            Self::The { plur } => plur.clone(),
            Self::A { .. } | Self::Each => Plurality::One,
            Self::All => Plurality::Many,
        }
    }
}
/// Discourse inspection of `SliceCount`.
pub trait SliceCountPhraseFactsExt: Sized {
    #[must_use]
    fn plurality(&self) -> Plurality;
}

impl SliceCountPhraseFactsExt for SliceCount {
    fn plurality(&self) -> Plurality {
        match self {
            Self::Counted { quantity } => quantity.plurality(),
            Self::Whole | Self::Totalling { .. } => Plurality::Many,
        }
    }
}
/// Discourse inspection of `NounPhrase`.
pub trait NounPhrasePhraseFactsExt: Sized {
    #[must_use]
    fn determiner(&self) -> Option<Determiner>;
    #[doc(hidden)]
    fn determiner_at(&self, fuel: u64) -> Option<Determiner>;
    #[must_use]
    fn plurality(&self) -> Plurality;
    #[doc(hidden)]
    fn plurality_at(&self, fuel: u64) -> Plurality;
}

impl NounPhrasePhraseFactsExt for NounPhrase {
    fn determiner(&self) -> Option<Determiner> {
        self.determiner_at(64)
    }
    fn determiner_at(&self, fuel: u64) -> Option<Determiner> {
        match (fuel, self) {
            (0, _) => None,
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::Agree { group: body, .. },
            ) => body.determiner_at(fuel - 1),
            (_, Self::Described { determiner, .. }) => Some(determiner.determiner()),
            (_, Self::EachOf { .. }) => Some(Determiner::Each),
            (_, Self::Slice { .. } | Self::Run { .. } | Self::TheRest { .. }) => {
                Some(Determiner::The)
            }
            (_, Self::SomeOf { .. } | Self::PileOf { .. }) => Some(Determiner::Part),
            (_, Self::OneEachOf { .. }) => Some(Determiner::A),
            _ => None,
        }
    }
    fn plurality(&self) -> Plurality {
        self.plurality_at(64)
    }
    fn plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (0, _) => Plurality::One,
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.plurality_at(fuel - 1),
            (_, Self::Described { determiner, .. }) => determiner.plurality(),
            (_, Self::SomeOf { count, .. } | Self::PileOf { count, .. }) => count.plurality(),
            (
                _,
                Self::TheRest { plurality }
                | Self::Pro { plurality, .. }
                | Self::Refl { plurality },
            ) => plurality.clone(),
            (_, Self::Slice { amount, pile, .. }) => {
                let owner = match pile.as_ref() {
                    Location::Zone {
                        whose: Some(who), ..
                    } => {
                        if who.determiner_at(fuel - 1) == Some(Determiner::Each) {
                            Plurality::Many
                        } else {
                            who.plurality_at(fuel - 1)
                        }
                    }
                    _ => Plurality::One,
                };
                if amount.plurality() == Plurality::One && owner == Plurality::One {
                    Plurality::One
                } else {
                    Plurality::Many
                }
            }
            (
                _,
                Self::Run { .. }
                | Self::OneEachOf { .. }
                | Self::Coord {
                    coordinator: Coordinator::And | Coordinator::AndOr,
                    ..
                },
            ) => Plurality::Many,
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or,
                    phrases,
                },
            ) => phrases
                .first()
                .map_or(Plurality::One, |p| p.plurality_at(fuel - 1)),
            _ => Plurality::One,
        }
    }
}
