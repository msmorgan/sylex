//! Predicate metadata used by the phrase checker, mirroring `Check/Phrase` and
//! `Check/PredicateShape`. Scope wrappers are preserved by the stepping layer.
use deckmaste_semantics_v3::ast::AttachmentSide;
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::ChoiceRef;
use deckmaste_semantics_v3::ast::CombatRelation;
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::CounterKind;
use deckmaste_semantics_v3::ast::DesignationLabel;
use deckmaste_semantics_v3::ast::DesignationScope;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::Location;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::PossessorAxis;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::ProjAxis;
use deckmaste_semantics_v3::ast::QualitySort;
use deckmaste_semantics_v3::ast::Zone;

use super::Determiner;
use super::Origin;
use super::Payload;
use super::Referent;
use super::common_types;
use super::merge_types;
use crate::reads::KindResolutionExt;
use crate::reads::SubtypeWordsExt;

/// Discourse inspection of `Location`.
pub trait LocationPredicateFactsExt: Sized {
    #[must_use]
    fn sort(&self) -> Option<Zone>;
    #[doc(hidden)]
    fn sort_at(&self, fuel: u64) -> Option<Zone>;
}

impl LocationPredicateFactsExt for Location {
    fn sort(&self) -> Option<Zone> {
        self.sort_at(64)
    }
    fn sort_at(&self, fuel: u64) -> Option<Zone> {
        if fuel == 0 {
            return None;
        }
        match (fuel, self) {
            (_, Self::InCaller { body } | Self::FromEnd { loc: body, .. }) => {
                body.sort_at(fuel - 1)
            }
            (_, Self::Zone { zone, .. }) => Some(zone.clone()),
            (_, Self::ExiledWith { .. }) => Some(Zone::Exile),
            (_, Self::Coord { locs, .. }) => {
                let zone = locs.first()?.sort_at(fuel - 1);
                if locs.iter().skip(1).all(|loc| loc.sort_at(fuel - 1) == zone) {
                    zone
                } else {
                    None
                }
            }
            (_, Self::Pile { .. } | Self::AnyLocation | Self::Not { .. }) => None,
        }
    }
}
/// Discourse inspection of `ProjAxis`.
pub trait ProjAxisPredicateFactsExt: Sized {
    #[must_use]
    fn scope(&self) -> Kind;
    #[must_use]
    fn multi_valued(&self) -> bool;
}

impl ProjAxisPredicateFactsExt for ProjAxis {
    fn scope(&self) -> Kind {
        match self {
            Self::Stat { .. } | Self::MarkedDamage | Self::Chapter | Self::KeywordNumber { .. } => {
                Kind::Object
            }
            Self::PlayerStat { .. } => Kind::Player,
            Self::Counter {
                kind: CounterKind::Named { holder, .. },
            }
            | Self::AnyCounter { kind: holder } => *holder.clone(),
            Self::BoundCounter => Kind::Join {
                left: Box::new(Kind::Object),
                right: Box::new(Kind::Player),
            },
        }
    }
    fn multi_valued(&self) -> bool {
        matches!(self, Self::Chapter | Self::KeywordNumber { .. })
    }
}
/// Discourse inspection of `DesignationLabel`.
pub trait DesignationLabelPredicateFactsExt: Sized {
    #[must_use]
    fn holder(&self) -> Option<Kind>;
}

impl DesignationLabelPredicateFactsExt for DesignationLabel {
    fn holder(&self) -> Option<Kind> {
        match self {
            Self::Named {
                scope: DesignationScope::HeldBy { holder },
                ..
            } => Some(*holder.clone()),
            Self::Named {
                scope: DesignationScope::HeldByCard,
                ..
            } => Some(Kind::Object),
            Self::Named {
                scope: DesignationScope::HeldByGame,
                ..
            } => None,
        }
    }
}
/// Discourse inspection of `Predicate`.
pub trait PredicateFactsExt: Sized {
    #[must_use]
    fn kind(&self) -> Option<Kind>;
    #[doc(hidden)]
    fn kind_at(&self, fuel: u64) -> Option<Kind>;
    #[must_use]
    fn scope_body(&self) -> &Self;
    #[must_use]
    fn conjuncts(&self) -> Vec<&Self>;
    #[doc(hidden)]
    fn conjuncts_at(&self, fuel: u64) -> Vec<&Self>;
    #[must_use]
    fn flattened(&self) -> Vec<Self>;
    #[doc(hidden)]
    fn flattened_at(&self, fuel: u64) -> Vec<Self>;
    #[must_use]
    fn seed_types(&self) -> Vec<CardType>;
    #[must_use]
    fn written_types(&self) -> Vec<CardType>;
    #[doc(hidden)]
    fn types_at(&self, fuel: u64, seed: bool) -> Vec<CardType>;
    #[must_use]
    fn head_type_alternatives(&self) -> Vec<Vec<CardType>>;
    #[doc(hidden)]
    fn head_type_alternatives_at(&self, fuel: u64) -> Vec<Vec<CardType>>;
    #[must_use]
    fn seed_zone(&self) -> Option<Zone>;
    #[must_use]
    fn explicit_zone(&self) -> Option<Zone>;
    #[doc(hidden)]
    fn zone_at(&self, fuel: u64, explicit: bool) -> Option<Zone>;
    #[doc(hidden)]
    fn conjunct_any(&self, fuel: u64, leaf: fn(&Self) -> bool) -> bool;
    #[must_use]
    fn seeds_token(&self) -> bool;
    #[must_use]
    fn head_is_placeless(&self) -> bool;
    #[must_use]
    fn demands_card(&self) -> bool;
    #[must_use]
    fn has_other(&self) -> bool;
    #[must_use]
    fn has_bare_other(&self) -> bool;
    #[must_use]
    fn seeds_ability(&self) -> bool;
    #[doc(hidden)]
    fn seeds_ability_at(&self, fuel: u64) -> bool;
    #[must_use]
    fn phrase_zone(&self, kind: &Kind) -> Option<Zone>;
    #[must_use]
    fn of_kind(&self, kind: &Kind) -> Self;
    #[must_use]
    fn payload(&self, kind: &Kind) -> Payload;
    #[must_use]
    fn binding(&self, det: Determiner, plur: Plurality, kind: &Kind) -> Referent;
    #[must_use]
    fn control_fact(&self) -> Option<bool>;
    #[doc(hidden)]
    fn control_fact_at(&self, fuel: u64) -> Option<bool>;
    #[must_use]
    fn opponent_controls(&self) -> bool;
    #[must_use]
    fn negatable(&self) -> bool;
    #[must_use]
    fn uniquifies(&self) -> bool;
    #[must_use]
    fn chosen_ref(&self) -> Option<&ChoiceRef>;
    #[doc(hidden)]
    fn chosen_ref_at(&self, fuel: u64) -> Option<&ChoiceRef>;
}

impl PredicateFactsExt for Predicate {
    fn kind(&self) -> Option<Kind> {
        self.kind_at(64)
    }
    fn kind_at(&self, fuel: u64) -> Option<Kind> {
        if fuel == 0 {
            return None;
        }
        match (fuel, self) {
            (_, Self::InCaller { body } | Self::Not { predicate: body }) => body.kind_at(fuel - 1),
            (
                _,
                Self::IsPlayer
                | Self::IsOpponentOf { .. }
                | Self::IsTeammateOf { .. }
                | Self::ChoseExtreme { .. },
            ) => Some(Kind::Player),
            (_, Self::QualityNoun { sort, .. }) => Some(Kind::Quality { sort: sort.clone() }),
            (_, Self::CounterKindOn { .. }) => Some(Kind::Quality {
                sort: QualitySort::CounterKind,
            }),
            (_, Self::HasDesignation { designation, .. }) => designation.holder(),
            (_, Self::Compare { axes, .. }) => axes.first().map(ProjAxis::scope),
            (_, Self::Superlative { axis, domain, .. }) => {
                domain.kind_at(fuel - 1).or_else(|| Some(axis.scope()))
            }
            (_, Self::CompareOver { domain, .. }) => domain.kind_at(fuel - 1),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().find_map(|p| p.kind_at(fuel - 1)),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or | Coordinator::AndOr,
                    items,
                },
            ) => items
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
            (
                _,
                Self::HasPossessor { .. }
                | Self::Did { .. }
                | Self::Other
                | Self::NotChosen
                | Self::Chosen { .. }
                | Self::OtherThan { .. }
                | Self::IsSource
                | Self::WithMostVotes
                | Self::CoinCameUp { .. }
                | Self::Attachment {
                    side: AttachmentSide::Host,
                    ..
                }
                | Self::InCombat {
                    relation: CombatRelation::AttackedBy,
                    ..
                }
                | Self::LegalAttachment {
                    side: AttachmentSide::Host,
                    ..
                },
            ) => None,
            _ => Some(Kind::Object),
        }
    }
    fn scope_body(&self) -> &Self {
        let mut p = self;
        for _ in 0..64 {
            match p {
                Self::InCaller { body } => p = body,
                _ => break,
            }
        }
        p
    }
    fn conjuncts(&self) -> Vec<&Self> {
        self.conjuncts_at(64)
    }
    fn conjuncts_at(&self, fuel: u64) -> Vec<&Self> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.conjuncts_at(fuel - 1),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items
                .iter()
                .flat_map(|p| p.conjuncts_at(fuel - 1))
                .collect(),
            _ => vec![self],
        }
    }
    fn flattened(&self) -> Vec<Self> {
        self.flattened_at(64)
    }
    fn flattened_at(&self, fuel: u64) -> Vec<Self> {
        match (fuel, self) {
            (0, _) => vec![],
            (_, Self::InCaller { body }) => body.flattened_at(fuel - 1),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items
                .iter()
                .flat_map(|p| p.flattened_at(fuel - 1))
                .collect(),
            (_, Self::Not { predicate }) => vec![Self::Not {
                predicate: Box::new(predicate.scope_body().clone()),
            }],
            _ => vec![self.clone()],
        }
    }
    fn seed_types(&self) -> Vec<CardType> {
        self.types_at(64, true)
    }
    fn written_types(&self) -> Vec<CardType> {
        self.types_at(64, false)
    }
    fn types_at(&self, fuel: u64, seed: bool) -> Vec<CardType> {
        match (fuel, self) {
            (0, _) => vec![],
            (_, Self::InCaller { body } | Self::CompareOver { domain: body, .. }) => {
                body.types_at(fuel - 1, seed)
            }
            (_, Self::Superlative { domain, .. }) if seed => domain.types_at(fuel - 1, seed),
            (_, Self::HasType { type_ }) => vec![type_.clone()],
            (_, Self::HasSubtype { subtype }) if seed => {
                subtype.card_type().cloned().into_iter().collect()
            }
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().rev().fold(vec![], |acc, p| {
                merge_types(&p.types_at(fuel - 1, seed), &acc)
            }),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or | Coordinator::AndOr,
                    items,
                },
            ) => {
                let Some((first, rest)) = items.split_first() else {
                    return vec![];
                };
                rest.iter().fold(first.types_at(fuel - 1, seed), |acc, p| {
                    common_types(&acc, &p.types_at(fuel - 1, seed))
                })
            }
            _ => vec![],
        }
    }
    fn head_type_alternatives(&self) -> Vec<Vec<CardType>> {
        self.head_type_alternatives_at(64)
    }
    fn head_type_alternatives_at(&self, fuel: u64) -> Vec<Vec<CardType>> {
        match (fuel, self) {
            (1.., Self::InCaller { body } | Self::CompareOver { domain: body, .. }) => {
                body.head_type_alternatives_at(fuel - 1)
            }
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().rev().fold(vec![vec![]], |right, p| {
                p.head_type_alternatives_at(fuel - 1)
                    .iter()
                    .flat_map(|left| right.iter().map(|r| merge_types(left, r)))
                    .collect()
            }),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::Or | Coordinator::AndOr,
                    items,
                },
            ) => items
                .iter()
                .flat_map(|p| p.head_type_alternatives_at(fuel - 1))
                .collect(),
            (1.., Self::HasType { type_ }) => vec![vec![type_.clone()]],
            (1.., Self::HasSubtype { subtype }) => {
                vec![subtype.card_type().cloned().into_iter().collect()]
            }
            _ => vec![vec![]],
        }
    }
    fn seed_zone(&self) -> Option<Zone> {
        self.zone_at(64, false)
    }
    fn explicit_zone(&self) -> Option<Zone> {
        self.zone_at(64, true)
    }
    fn zone_at(&self, fuel: u64, explicit: bool) -> Option<Zone> {
        if fuel == 0 {
            return None;
        }
        match (fuel, self) {
            (_, Self::InCaller { body }) => body.zone_at(fuel - 1, explicit),
            (_, Self::In { loc }) => loc.sort(),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().find_map(|p| p.zone_at(fuel - 1, explicit)),
            (
                _,
                Self::Coord {
                    coordinator: Coordinator::Or | Coordinator::AndOr,
                    items,
                },
            ) => {
                let zone = items.first()?.zone_at(fuel - 1, explicit);
                if items
                    .iter()
                    .skip(1)
                    .all(|p| p.zone_at(fuel - 1, explicit) == zone)
                {
                    zone
                } else {
                    None
                }
            }
            _ if explicit => None,
            (
                _,
                Self::InCombat {
                    relation: CombatRelation::AttackedBy,
                    ..
                },
            ) => None,
            (
                _,
                Self::IsToken
                | Self::InCombat { .. }
                | Self::Attachment { .. }
                | Self::CurrentFace { .. }
                | Self::HasStatus { .. },
            ) => Some(Zone::Battlefield),
            (_, Self::AbilityHead { .. } | Self::AbilityOf { .. } | Self::Targets { .. }) => {
                Some(Zone::Stack)
            }
            (_, Self::IsEmblem) => Some(Zone::Command),
            _ => None,
        }
    }
    fn conjunct_any(&self, fuel: u64, leaf: fn(&Self) -> bool) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.conjunct_any(fuel - 1, leaf),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().any(|p| p.conjunct_any(fuel - 1, leaf)),
            (1.., p) => leaf(p),
            _ => false,
        }
    }
    fn seeds_token(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::IsToken))
    }
    fn head_is_placeless(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::IsSource | Self::IsCard))
    }
    fn demands_card(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::IsCard))
    }
    fn has_other(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::Other | Self::OtherThan { .. }))
    }
    fn has_bare_other(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::Other))
    }
    fn seeds_ability(&self) -> bool {
        self.seeds_ability_at(64)
    }
    fn seeds_ability_at(&self, fuel: u64) -> bool {
        match (fuel, self) {
            (1.., Self::InCaller { body } | Self::CompareOver { domain: body, .. }) => {
                body.seeds_ability_at(fuel - 1)
            }
            (
                1..,
                Self::AbilityHead { .. } | Self::AbilityOf { .. } | Self::TriggeredNotLeftStack,
            ) => true,
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().any(|p| p.seeds_ability_at(fuel - 1)),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::Or | Coordinator::AndOr,
                    items,
                },
            ) => !items.is_empty() && items.iter().all(|p| p.seeds_ability_at(fuel - 1)),
            _ => false,
        }
    }
    fn phrase_zone(&self, kind: &Kind) -> Option<Zone> {
        if !kind.lte(&Kind::Object) {
            None
        } else if self.head_is_placeless() {
            self.explicit_zone()
        } else {
            self.seed_zone().or(Some(Zone::Battlefield))
        }
    }
    fn of_kind(&self, kind: &Kind) -> Self {
        match self {
            Self::Coord {
                coordinator: Coordinator::Or | Coordinator::AndOr,
                items,
            } => {
                let Self::Coord { coordinator, .. } = self else { unreachable!() };
                Self::Coord {
                    coordinator: coordinator.clone(),
                    items: items
                        .iter()
                        .filter(|p| p.kind().is_none_or(|k| k == *kind))
                        .cloned()
                        .collect(),
                }
            }
            _ => self.clone(),
        }
    }
    fn payload(&self, kind: &Kind) -> Payload {
        match kind {
            Kind::Object => {
                let p = self.of_kind(kind);
                if p.seeds_ability() {
                    Payload::Ability {
                        orig: None,
                        zone: p.explicit_zone().or(Some(Zone::Stack)),
                    }
                } else {
                    Payload::Object {
                        ty: p.seed_types(),
                        zone: p.phrase_zone(kind),
                        prov: None,
                        orig: if p.seeds_token() { Some(Origin::Token) } else { None },
                        size: None,
                    }
                }
            }
            Kind::Player => Payload::Player { chosen: false },
            Kind::Quality { sort } => Payload::Quality { q: sort.clone() },
            Kind::Letter { letter } => Payload::Letter { l: letter.clone() },
            Kind::TurnRef => Payload::TurnRef,
            Kind::Pile => Payload::Pile {
                zone: None,
                size: None,
                face: None,
            },
            Kind::Join { left, right } => Payload::Join {
                l: Box::new(self.payload(left)),
                r: Box::new(self.payload(right)),
            },
            Kind::Outcome => Payload::Difference,
        }
    }
    fn binding(&self, det: Determiner, plur: Plurality, kind: &Kind) -> Referent {
        let mut binding = Referent::new(det, plur, self.payload(kind));
        binding.introduced_as_source = self.conjuncts().iter().any(|p| matches!(p, Self::IsSource));
        binding
    }
    fn control_fact(&self) -> Option<bool> {
        self.control_fact_at(64)
    }
    fn control_fact_at(&self, fuel: u64) -> Option<bool> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.control_fact_at(fuel - 1),
            (
                1..,
                Self::HasPossessor {
                    axis: PossessorAxis::Controller,
                    possessor,
                },
            ) => {
                if **possessor == NounPhrase::You {
                    Some(true)
                } else if possessor.names_opponent() {
                    Some(false)
                } else {
                    None
                }
            }
            (1.., Self::Not { predicate }) if matches!(predicate.as_ref(), Self::HasPossessor { axis: PossessorAxis::Controller, possessor } if **possessor == NounPhrase::You) => {
                Some(false)
            }
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().find_map(|p| p.control_fact_at(fuel - 1)),
            _ => None,
        }
    }
    fn opponent_controls(&self) -> bool {
        self.conjunct_any(64, |p| matches!(p, Self::HasPossessor { axis: PossessorAxis::Controller, possessor } if possessor.names_opponent()))
    }
    fn negatable(&self) -> bool {
        let mut p = self;
        for _ in 0..64 {
            match p {
                Self::InCaller { body } => p = body,
                Self::IsPlayer
                | Self::Chosen { .. }
                | Self::QualityNoun { domain: None, .. }
                | Self::CounterKindOn { .. }
                | Self::IsSource
                | Self::EachKind { .. } => return false,
                _ => return true,
            }
        }
        true
    }
    fn uniquifies(&self) -> bool {
        self.conjunct_any(64, |p| match p {
            Self::Chosen { .. }
            | Self::Superlative { .. }
            | Self::InCombat {
                relation: CombatRelation::AttackedBy,
                ..
            }
            | Self::Related {
                counterpart: Some(_),
                ..
            } => true,
            Self::In { loc } => matches!(loc.as_ref(), Location::ExiledWith { .. }),
            Self::Did { clause } => clause.ordinal.is_some(),
            Self::HasDesignation {
                designation: DesignationLabel::Named { sole, .. },
                holder: None,
            } => *sole,
            _ => false,
        })
    }
    fn chosen_ref(&self) -> Option<&ChoiceRef> {
        self.chosen_ref_at(64)
    }
    fn chosen_ref_at(&self, fuel: u64) -> Option<&ChoiceRef> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.chosen_ref_at(fuel - 1),
            (1.., Self::Chosen { ref_ }) => Some(ref_),
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => items.iter().find_map(|p| p.chosen_ref_at(fuel - 1)),
            _ => None,
        }
    }
}
/// Discourse inspection of `NounPhrase`.
pub trait NounPhrasePredicateFactsExt: Sized {
    #[must_use]
    fn names_opponent(&self) -> bool;
}

impl NounPhrasePredicateFactsExt for NounPhrase {
    fn names_opponent(&self) -> bool {
        matches!(self, Self::Described { predicate, .. } if matches!(predicate.as_ref(), Predicate::IsOpponentOf { who } if **who == Self::You))
    }
}
