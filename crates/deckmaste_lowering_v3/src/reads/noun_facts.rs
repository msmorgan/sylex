//! Noun-phrase shape and kind facts from `Check/Phrase`.
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::DetPhrase;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::NounWord;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Reach;
use deckmaste_semantics_v3::ast::SlotCarrier;

use super::DRS;
use super::Determiner;
use super::ReferentId;
use crate::reads::NounPhrasePhraseFactsExt;
use crate::reads::NounWordAnchorsExt;
use crate::reads::PredicateAnchorsExt;
use crate::reads::PredicateFactsExt;
use crate::reads::ReachAnchorsExt;
/// Discourse inspection of `NounPhrase`.
pub trait NounPhraseFactsExt: Sized {
    #[must_use]
    fn kind(&self) -> Option<Kind>;
    #[doc(hidden)]
    fn kind_at(&self, fuel: u64) -> Option<Kind>;
    #[must_use]
    fn kind_in(&self, drs: &DRS, default: Kind) -> Kind;
    #[doc(hidden)]
    fn kind_in_at(&self, drs: &DRS, default: Kind, fuel: u64) -> Kind;
    #[must_use]
    fn selected_kind(&self, drs: &DRS, ids: &[ReferentId]) -> Kind;
    fn selection_kind_override(&self, fuel: u64) -> Option<Kind>;
    #[must_use]
    fn agent_choosable(&self) -> bool;
    #[doc(hidden)]
    fn agent_choosable_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn per_member_ok(&self) -> bool;
    #[must_use]
    fn anchor_phrase(&self) -> bool;
    #[doc(hidden)]
    fn anchor_phrase_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn group_mention(&self) -> bool;
    #[doc(hidden)]
    fn group_mention_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn countable_group(&self) -> bool;
    #[must_use]
    fn existential_mention(&self) -> bool;
    #[doc(hidden)]
    fn existential_mention_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn pile_mention(&self) -> bool;
    #[doc(hidden)]
    fn pile_mention_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn partitive_base(&self) -> bool;
    #[doc(hidden)]
    fn partitive_base_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn player_group_mention(&self) -> bool;
    #[doc(hidden)]
    fn player_group_mention_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn sole_holder_ok(&self) -> bool;
    #[must_use]
    fn slice_possessor_ok(&self) -> bool;
    #[must_use]
    fn event_agent_ok(&self) -> bool;
    #[doc(hidden)]
    fn event_agent_ok_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn token_phrase(&self) -> bool;
    #[doc(hidden)]
    fn token_phrase_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn narrow_bare_to(&self, carrier: &SlotCarrier) -> Self;
    #[doc(hidden)]
    #[must_use]
    fn narrow_bare_to_at(&self, carrier: &SlotCarrier, fuel: u64) -> Self;
    #[must_use]
    fn self_defined_ok(&self) -> bool;
    #[must_use]
    fn ascribable_in(&self, drs: &DRS) -> bool;
    #[doc(hidden)]
    fn ascribable_at(&self, drs: &DRS, fuel: u64) -> bool;
    #[must_use]
    fn has_bare_other(&self) -> bool;
}

impl NounPhraseFactsExt for NounPhrase {
    fn kind(&self) -> Option<Kind> {
        self.kind_at(64)
    }
    fn kind_at(&self, fuel: u64) -> Option<Kind> {
        match (fuel, self) {
            (0, _) => Some(Kind::Object),
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.kind_at(fuel - 1),
            (_, Self::You | Self::Actor | Self::PossessorOf { .. } | Self::CombatPlayer { .. }) => {
                Some(Kind::Player)
            }
            (_, Self::SomeOf { group, .. }) => group.kind_at(fuel - 1),
            (_, Self::PileOf { .. }) => Some(Kind::Pile),
            (_, Self::AttachHost { head, .. }) => Some(head.kind()),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or,
                    phrases,
                },
            ) => phrases
                .iter()
                .filter_map(|p| p.kind_at(fuel - 1))
                .reduce(|left, right| {
                    if left == right {
                        left
                    } else {
                        Kind::Join {
                            left: Box::new(left),
                            right: Box::new(right),
                        }
                    }
                }),
            (_, Self::Described { predicate, .. }) => predicate.kind(),
            (_, Self::Pro { reach, .. }) => Some(reach.kind()),
            (_, Self::TheRest { .. }) => None,
            _ => Some(Kind::Object),
        }
    }
    fn kind_in(&self, drs: &DRS, default: Kind) -> Kind {
        self.kind_in_at(drs, default, 64)
    }
    fn kind_in_at(&self, drs: &DRS, default: Kind, fuel: u64) -> Kind {
        match (fuel, self) {
            (0, _) => default,
            (_, Self::InCaller { body }) => {
                let mut caller = drs.clone();
                caller.enter_caller();
                body.kind_in_at(&caller, default, fuel - 1)
            }
            (_, Self::FirstMention { phrase, .. } | Self::LaterMention { phrase, .. }) => {
                phrase.kind_in_at(drs, default, fuel - 1)
            }
            (_, Self::Actor) => drs
                .performers
                .first()
                .map_or(Kind::Player, |shape| *shape.kind.clone()),
            (_, Self::Which) => drs
                .relative_binder
                .and_then(|id| drs.find_by_id(id))
                .map_or(default, |r| r.payload.kind()),
            (_, Self::Refl { plurality }) => drs
                .resolve_reflexive(plurality)
                .first()
                .map_or(default, |r| r.payload.kind()),
            (_, Self::TheRest { plurality }) => drs.rest_kind(plurality).unwrap_or(default),
            _ => self.kind().unwrap_or(default),
        }
    }
    fn selected_kind(&self, drs: &DRS, ids: &[ReferentId]) -> Kind {
        self.selection_kind_override(64)
            .or_else(|| {
                ids.first()
                    .and_then(|id| drs.find_by_id(*id))
                    .map(|r| r.payload.kind())
            })
            .unwrap_or_else(|| self.kind_in(drs, Kind::Object))
    }
    fn selection_kind_override(&self, fuel: u64) -> Option<Kind> {
        match (fuel, self) {
            (
                1..,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.selection_kind_override(fuel - 1),
            (
                1..,
                Self::Pro {
                    reach: Reach::UnionHalf { word } | Reach::Word { word },
                    ..
                },
            ) => Some(word.kind()),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::Or,
                    ..
                },
            ) => self.kind(),
            _ => None,
        }
    }
    fn agent_choosable(&self) -> bool {
        self.agent_choosable_at(64)
    }
    fn agent_choosable_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (_, Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.agent_choosable_at(fuel - 1)
            }
            (_, Self::SomeOf { .. } | Self::PileOf { .. }) => true,
            (_, Self::Coord { phrases, .. }) => {
                !phrases.is_empty() && phrases.iter().all(|p| p.agent_choosable_at(fuel - 1))
            }
            _ => matches!(
                self.determiner(),
                Some(Determiner::A | Determiner::Target | Determiner::Count)
            ),
        }
    }
    fn per_member_ok(&self) -> bool {
        matches!(self.determiner(), Some(Determiner::Each | Determiner::All))
            || self.plurality() == Plurality::One
    }
    fn anchor_phrase(&self) -> bool {
        self.anchor_phrase_at(64)
    }
    fn anchor_phrase_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (_, Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.anchor_phrase_at(fuel - 1)
            }
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or,
                    phrases,
                },
            ) => phrases.iter().all(|p| p.anchor_phrase_at(fuel - 1)),
            (_, Self::Coord { .. }) => false,
            (
                _,
                Self::Described {
                    determiner,
                    predicate,
                },
            ) if **determiner
                == (DetPhrase::The {
                    plur: Plurality::One,
                }) =>
            {
                predicate.participial_anchor().is_some()
            }
            _ => self.determiner().is_none_or(|d| d == Determiner::Target),
        }
    }
    fn group_mention(&self) -> bool {
        self.group_mention_at(64)
    }
    fn group_mention_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (_, Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.group_mention_at(fuel - 1)
            }
            (_, Self::LaterMention { phrase, .. }) => phrase.plurality() == Plurality::Many,
            (
                _,
                Self::Slice { .. }
                | Self::Run { .. }
                | Self::OneEachOf { .. }
                | Self::Coord {
                    coordinator: Coordinator::And | Coordinator::AndOr,
                    ..
                },
            ) => true,
            (_, Self::Pro { plurality, .. }) => *plurality == Plurality::Many,
            (_, Self::Described { determiner, .. })
                if matches!(determiner.as_ref(), DetPhrase::The { .. }) =>
            {
                matches!(
                    determiner.as_ref(),
                    DetPhrase::The {
                        plur: Plurality::Many
                    }
                )
            }
            _ => self.determiner() == Some(Determiner::Target),
        }
    }
    fn countable_group(&self) -> bool {
        matches!(self.determiner(), Some(Determiner::All | Determiner::Count))
            || self.group_mention()
    }
    fn existential_mention(&self) -> bool {
        self.existential_mention_at(64)
    }
    fn existential_mention_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (_, Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.existential_mention_at(fuel - 1)
            }
            (_, Self::Agree { group, .. }) => matches!(
                group.determiner(),
                Some(Determiner::Count | Determiner::Target)
            ),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::And,
                    phrases,
                },
            ) => !phrases.is_empty() && phrases.iter().all(|p| p.existential_mention_at(fuel - 1)),
            _ => matches!(self.determiner(), Some(Determiner::A | Determiner::Count)),
        }
    }
    fn pile_mention(&self) -> bool {
        self.pile_mention_at(64)
    }
    fn pile_mention_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (
                1..,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.pile_mention_at(fuel - 1),
            (1.., Self::PileOf { .. }) => true,
            (
                1..,
                Self::Pro {
                    reach: Reach::Word { word },
                    ..
                },
            ) => **word == NounWord::Pile,
            _ => false,
        }
    }
    fn partitive_base(&self) -> bool {
        self.partitive_base_at(64)
    }
    fn partitive_base_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (_, Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.partitive_base_at(fuel - 1)
            }
            (_, Self::LaterMention { .. }) => true,
            _ => self.determiner() == Some(Determiner::All) || self.group_mention(),
        }
    }
    fn player_group_mention(&self) -> bool {
        self.player_group_mention_at(64)
    }
    fn player_group_mention_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (
                1..,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. },
            ) => body.player_group_mention_at(fuel - 1),
            (
                1..,
                Self::Described {
                    determiner,
                    predicate,
                },
            ) => **determiner == DetPhrase::All && predicate.kind() == Some(Kind::Player),
            _ => false,
        }
    }
    fn sole_holder_ok(&self) -> bool {
        self.player_group_mention() || self.plurality() == Plurality::One
    }
    fn slice_possessor_ok(&self) -> bool {
        self.determiner() == Some(Determiner::Each) || self.sole_holder_ok()
    }
    fn event_agent_ok(&self) -> bool {
        self.event_agent_ok_at(64)
    }
    fn event_agent_ok_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (0, _) => false,
            (
                _,
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::Headed { subject: body, .. },
            ) => body.event_agent_ok_at(fuel - 1),
            (
                _,
                Self::You
                | Self::This
                | Self::Actor
                | Self::Pro { .. }
                | Self::Which
                | Self::Refl { .. }
                | Self::LaterMention { .. },
            ) => true,
            _ => self.player_group_mention(),
        }
    }
    fn token_phrase(&self) -> bool {
        self.token_phrase_at(64)
    }
    fn token_phrase_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body } | Self::FirstMention { phrase: body, .. }) => {
                body.token_phrase_at(fuel - 1)
            }
            (
                1..,
                Self::Described {
                    determiner,
                    predicate,
                },
            ) if matches!(
                determiner.as_ref(),
                DetPhrase::Count { .. } | DetPhrase::A { .. }
            ) =>
            {
                predicate.seeds_token()
            }
            _ => false,
        }
    }
    fn narrow_bare_to(&self, carrier: &SlotCarrier) -> Self {
        self.narrow_bare_to_at(carrier, 64)
    }
    fn narrow_bare_to_at(&self, carrier: &SlotCarrier, fuel: u64) -> Self {
        match (fuel, self) {
            (
                1..,
                Self::Pro {
                    reach: Reach::Bare,
                    plurality,
                },
            ) => Self::Pro {
                reach: Reach::AtSlot {
                    slot: carrier.clone(),
                },
                plurality: plurality.clone(),
            },
            (1.., Self::InCaller { body }) => Self::InCaller {
                body: Box::new(body.narrow_bare_to_at(carrier, fuel - 1)),
            },
            (1.., Self::FirstMention { id, phrase }) => Self::FirstMention {
                id: *id,
                phrase: Box::new(phrase.narrow_bare_to_at(carrier, fuel - 1)),
            },
            (1.., Self::LaterMention { id, phrase }) => Self::LaterMention {
                id: *id,
                phrase: Box::new(phrase.narrow_bare_to_at(carrier, fuel - 1)),
            },
            _ => self.clone(),
        }
    }

    fn self_defined_ok(&self) -> bool {
        let mut noun = self;
        for _ in 0..64 {
            match noun {
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. } => noun = body,
                Self::Headed { head, subject }
                    if matches!(
                        head.as_ref(),
                        NounWord::Type { .. } | NounWord::Subtype { .. }
                    ) =>
                {
                    noun = subject;
                }
                Self::This => return true,
                _ => return false,
            }
        }
        false
    }

    fn ascribable_in(&self, drs: &DRS) -> bool {
        self.ascribable_at(drs, 64)
    }
    fn ascribable_at(&self, drs: &DRS, fuel: u64) -> bool {
        if fuel == 0 {
            return false;
        }
        match self {
            Self::InCaller { body } => {
                let mut caller = drs.clone();
                caller.enter_caller();
                body.ascribable_at(&caller, fuel - 1)
            }
            Self::FirstMention { phrase, .. } | Self::LaterMention { phrase, .. } => {
                phrase.ascribable_at(drs, fuel - 1)
            }
            Self::This | Self::Designated { .. } | Self::Which => true,
            Self::Actor => drs.performers.first().is_some_and(|actor| actor.ascribable),
            _ => false,
        }
    }

    fn has_bare_other(&self) -> bool {
        let mut noun = self;
        for _ in 0..64 {
            match noun {
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::Headed { subject: body, .. } => noun = body,
                Self::Described { predicate, .. } => return predicate.has_bare_other(),
                _ => return false,
            }
        }
        false
    }
}
