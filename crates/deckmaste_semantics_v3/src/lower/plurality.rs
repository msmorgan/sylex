//! Syntactic plurality used by bundle authoring expressions.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Determiner {
    Target,
    A,
    Each,
    All,
    The,
    Part,
    Count,
}
use crate::ast::Amount;
use crate::ast::Coordinator;
use crate::ast::DetPhrase;
use crate::ast::Location;
use crate::ast::NounPhrase;
use crate::ast::Plurality;
use crate::ast::Quantity;
use crate::ast::SliceCount;

impl Amount {
    #[must_use]
    fn authoring_plurality(&self) -> Plurality {
        self.authoring_plurality_at(64)
    }
    fn authoring_plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.authoring_plurality_at(fuel - 1),
            (1.., Self::Lit { value: 1 } | Self::TheNth { .. }) => Plurality::One,
            _ => Plurality::Many,
        }
    }
}
impl Quantity {
    #[must_use]
    fn authoring_plurality(&self) -> Plurality {
        self.authoring_plurality_at(64)
    }
    fn authoring_plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (0, _) => Plurality::Many,
            (_, Self::InCaller { body } | Self::Additional { quantity: body }) => {
                body.authoring_plurality_at(fuel - 1)
            }
            (_, Self::Range { high: Some(1), .. } | Self::TheLast) => Plurality::One,
            (_, Self::ExactlyOf { amount } | Self::UpToOf { amount }) => {
                amount.authoring_plurality_at(fuel - 1)
            }
            _ => Plurality::Many,
        }
    }
}
impl DetPhrase {
    #[must_use]
    fn authoring_determiner(&self) -> Determiner {
        match self {
            Self::Target { .. } => Determiner::Target,
            Self::A { .. } => Determiner::A,
            Self::Each => Determiner::Each,
            Self::All => Determiner::All,
            Self::The { .. } => Determiner::The,
            Self::Count { .. } => Determiner::Count,
        }
    }
    #[must_use]
    fn authoring_plurality(&self) -> Plurality {
        match self {
            Self::Target { quantity } | Self::Count { quantity, .. } => {
                quantity.authoring_plurality()
            }
            Self::The { plur } => plur.clone(),
            Self::A { .. } | Self::Each => Plurality::One,
            Self::All => Plurality::Many,
        }
    }
}
impl SliceCount {
    #[must_use]
    fn authoring_plurality(&self) -> Plurality {
        match self {
            Self::Counted { quantity } => quantity.authoring_plurality(),
            Self::Whole | Self::Totalling { .. } => Plurality::Many,
        }
    }
}
impl NounPhrase {
    fn authoring_determiner_at(&self, fuel: u64) -> Option<Determiner> {
        match (fuel, self) {
            (0, _) => None,
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::Agree { group: body, .. },
            ) => body.authoring_determiner_at(fuel - 1),
            (_, Self::Described { determiner, .. }) => Some(determiner.authoring_determiner()),
            (_, Self::EachOf { .. }) => Some(Determiner::Each),
            (_, Self::Slice { .. } | Self::Run { .. } | Self::TheRest { .. }) => {
                Some(Determiner::The)
            }
            (_, Self::SomeOf { .. } | Self::PileOf { .. }) => Some(Determiner::Part),
            (_, Self::OneEachOf { .. }) => Some(Determiner::A),
            _ => None,
        }
    }
    #[must_use]
    fn authoring_plurality(&self) -> Plurality {
        self.authoring_plurality_at(64)
    }
    fn authoring_plurality_at(&self, fuel: u64) -> Plurality {
        match (fuel, self) {
            (0, _) => Plurality::One,
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.authoring_plurality_at(fuel - 1),
            (_, Self::Described { determiner, .. }) => determiner.authoring_plurality(),
            (_, Self::SomeOf { count, .. } | Self::PileOf { count, .. }) => {
                count.authoring_plurality()
            }
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
                        if who.authoring_determiner_at(fuel - 1) == Some(Determiner::Each) {
                            Plurality::Many
                        } else {
                            who.authoring_plurality_at(fuel - 1)
                        }
                    }
                    _ => Plurality::One,
                };
                if amount.authoring_plurality() == Plurality::One && owner == Plurality::One {
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
                .map_or(Plurality::One, |p| p.authoring_plurality_at(fuel - 1)),
            _ => Plurality::One,
        }
    }
}

pub(super) fn noun_plurality(noun: &NounPhrase) -> Plurality {
    noun.authoring_plurality()
}
