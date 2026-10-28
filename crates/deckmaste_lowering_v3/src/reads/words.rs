//! Scalar and payload facts from `Semantics/Check/Words.lean`.
use deckmaste_semantics_v3::ast::CardClass;
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::ChoiceRef;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::Delta;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::PileFace;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::QualitySort;
use deckmaste_semantics_v3::ast::Stat;
use deckmaste_semantics_v3::ast::Subtype;

use super::DRS;
use super::Determiner;
use super::Origin;
use super::Payload;
use super::Referent;
use super::Stamp;
use crate::reads::KindResolutionExt;

/// Discourse inspection of `CardType`.
pub trait CardTypeWordsExt: Sized {
    #[must_use]
    fn creature() -> Self;
    #[must_use]
    fn artifact() -> Self;
    #[must_use]
    fn land() -> Self;
    #[must_use]
    fn enchantment() -> Self;
    #[must_use]
    fn instant() -> Self;
    #[must_use]
    fn sorcery() -> Self;
    #[must_use]
    fn planeswalker() -> Self;
    #[must_use]
    fn battle() -> Self;
    #[must_use]
    fn kindred() -> Self;
    #[must_use]
    fn dungeon() -> Self;
    #[doc(hidden)]
    fn plain(name: &str, card_class: Option<CardClass>) -> Self;
    #[must_use]
    fn all() -> [Self; 10];
}

impl CardTypeWordsExt for CardType {
    fn creature() -> Self {
        Self {
            name: "Creature".into(),
            card_class: Some(CardClass::PermanentCard),
            stats: vec![Stat::Power, Stat::Toughness],
        }
    }
    fn artifact() -> Self {
        Self::plain("Artifact", Some(CardClass::PermanentCard))
    }
    fn land() -> Self {
        Self::plain("Land", Some(CardClass::PermanentCard))
    }
    fn enchantment() -> Self {
        Self::plain("Enchantment", Some(CardClass::PermanentCard))
    }
    fn instant() -> Self {
        Self::plain("Instant", Some(CardClass::InstantOrSorceryCard))
    }
    fn sorcery() -> Self {
        Self::plain("Sorcery", Some(CardClass::InstantOrSorceryCard))
    }
    fn planeswalker() -> Self {
        Self {
            stats: vec![Stat::Loyalty],
            ..Self::plain("Planeswalker", Some(CardClass::PermanentCard))
        }
    }
    fn battle() -> Self {
        Self {
            stats: vec![Stat::Defense],
            ..Self::plain("Battle", Some(CardClass::PermanentCard))
        }
    }
    fn kindred() -> Self {
        Self::plain("Kindred", None)
    }
    fn dungeon() -> Self {
        Self::plain("Dungeon", None)
    }
    fn plain(name: &str, card_class: Option<CardClass>) -> Self {
        Self {
            name: name.into(),
            card_class,
            stats: vec![],
        }
    }
    fn all() -> [Self; 10] {
        [
            Self::creature(),
            Self::artifact(),
            Self::land(),
            Self::enchantment(),
            Self::instant(),
            Self::sorcery(),
            Self::planeswalker(),
            Self::battle(),
            Self::kindred(),
            Self::dungeon(),
        ]
    }
}
#[must_use]
pub fn normalize_types(types: &[CardType]) -> Vec<CardType> {
    CardType::all()
        .into_iter()
        .filter(|ty| types.contains(ty))
        .collect()
}
#[must_use]
pub fn merge_types(left: &[CardType], right: &[CardType]) -> Vec<CardType> {
    CardType::all()
        .into_iter()
        .filter(|ty| left.contains(ty) || right.contains(ty))
        .collect()
}
#[must_use]
pub fn common_types(left: &[CardType], right: &[CardType]) -> Vec<CardType> {
    CardType::all()
        .into_iter()
        .filter(|ty| left.contains(ty) && right.contains(ty))
        .collect()
}
fn join_seed(left: Vec<CardType>, right: Vec<CardType>) -> Vec<CardType> {
    if left.is_empty() {
        right
    } else if right.is_empty() {
        left
    } else {
        common_types(&left, &right)
    }
}
/// Discourse inspection of `QualitySort`.
pub trait QualitySortWordsExt: Sized {
    #[must_use]
    fn chosen_read_ok(&self) -> bool;
}

impl QualitySortWordsExt for QualitySort {
    fn chosen_read_ok(&self) -> bool {
        !matches!(self, Self::Number | Self::CounterKind)
    }
}
/// Discourse inspection of `OutcomeSort`.
pub trait OutcomeSortWordsExt: Sized {
    #[must_use]
    fn comparable(&self) -> bool;
    #[must_use]
    fn is_amount(&self) -> bool;
}

impl OutcomeSortWordsExt for OutcomeSort {
    fn comparable(&self) -> bool {
        matches!(self, Self::RollResult | Self::DamageMarked)
    }
    fn is_amount(&self) -> bool {
        !matches!(
            self,
            Self::CoinFlipped
                | Self::ManaAdded
                | Self::ManaProduced
                | Self::CeilingShortfall
                | Self::VoteHeld
        )
    }
}
/// Discourse inspection of `Delta`.
pub trait DeltaWordsExt<T>: Sized {
    fn amount(&self) -> &T;
    fn is_set(&self) -> bool;
}

impl<T> DeltaWordsExt<T> for Delta<T> {
    fn amount(&self) -> &T {
        match self {
            Self::Up { amount } | Self::Down { amount } | Self::Set { amount } => amount,
        }
    }
    fn is_set(&self) -> bool {
        matches!(self, Self::Set { .. })
    }
}
/// Discourse inspection of `Subtype`.
pub trait SubtypeWordsExt: Sized {
    #[must_use]
    fn card_type(&self) -> Option<&CardType>;
    #[must_use]
    fn fits(&self, ty: &CardType) -> bool;
}

impl SubtypeWordsExt for Subtype {
    fn card_type(&self) -> Option<&CardType> {
        match self {
            Self::Of { host, .. } => Some(host),
            Self::Spell { .. } => None,
        }
    }
    fn fits(&self, ty: &CardType) -> bool {
        match self {
            Self::Of { host, .. } => host == ty,
            Self::Spell { .. } => ty.card_class == Some(CardClass::InstantOrSorceryCard),
        }
    }
}
impl Payload {
    #[must_use]
    pub fn is_choice(&self) -> bool {
        matches!(self, Self::Quality { .. } | Self::Player { chosen: true })
    }
    #[must_use]
    pub fn types(&self) -> Vec<CardType> {
        match self {
            Self::Object { ty, .. } => ty.clone(),
            Self::Join { l, r } => join_seed(l.types(), r.types()),
            _ => vec![],
        }
    }
    #[must_use]
    pub fn size(&self) -> Option<u64> {
        match self {
            Self::Object { size, .. } | Self::Pile { size, .. } => *size,
            Self::Join { l, r } => l.size().or_else(|| r.size()),
            _ => None,
        }
    }
    #[must_use]
    pub fn face(&self) -> Option<&PileFace> {
        match self {
            Self::Pile { face, .. } => face.as_ref(),
            _ => None,
        }
    }
    #[must_use]
    pub fn provenance(&self) -> Option<&Stamp> {
        match self {
            Self::Object { prov, .. } => prov.as_ref(),
            Self::Join { l, r } => l.provenance().or_else(|| r.provenance()),
            _ => None,
        }
    }
    #[must_use]
    pub fn origin(&self) -> Option<&Origin> {
        match self {
            Self::Object { orig, .. } | Self::Ability { orig, .. } => orig.as_ref(),
            Self::Join { l, r } => l.origin().or_else(|| r.origin()),
            _ => None,
        }
    }
    #[must_use]
    pub fn together(&self, other: &Self) -> Self {
        match (self, other) {
            (
                Self::Object {
                    ty,
                    zone,
                    prov,
                    orig,
                    ..
                },
                Self::Object {
                    ty: ts,
                    zone: zs,
                    prov: ps,
                    orig: os,
                    ..
                },
            ) => Self::Object {
                ty: join_seed(ty.clone(), ts.clone()),
                zone: if zone == zs { zone.clone() } else { None },
                prov: if prov == ps { prov.clone() } else { None },
                orig: if orig == os { orig.clone() } else { None },
                size: None,
            },
            (Self::Player { chosen }, Self::Player { chosen: other }) => Self::Player {
                chosen: *chosen && *other,
            },
            _ => Self::Join {
                l: Box::new(self.clone()),
                r: Box::new(other.clone()),
            },
        }
    }
    #[must_use]
    pub fn group_of(payloads: &[Self]) -> Option<Self> {
        payloads.split_first().map(|(first, rest)| {
            rest.iter()
                .fold(first.clone(), |group, next| group.together(next))
        })
    }
}
/// Discourse inspection of `ChoiceSort`.
pub trait ChoiceSortWordsExt: Sized {
    #[must_use]
    fn at(kind: &Kind) -> Option<Self>;
    #[must_use]
    fn binding(&self) -> Referent;
    #[must_use]
    fn binds(&self, kind: &Kind, payload: &Payload) -> bool;
}

impl ChoiceSortWordsExt for ChoiceSort {
    fn at(kind: &Kind) -> Option<Self> {
        match kind {
            Kind::Quality { sort } => Some(Self::Quality { sort: sort.clone() }),
            Kind::Player => Some(Self::Player),
            _ => None,
        }
    }
    fn binding(&self) -> Referent {
        let payload = match self {
            Self::Quality { sort } => Payload::Quality { q: sort.clone() },
            Self::Player => Payload::Player { chosen: true },
        };
        Referent::new(Determiner::A, Plurality::One, payload)
    }
    fn binds(&self, kind: &Kind, payload: &Payload) -> bool {
        match self {
            Self::Quality { sort } => Kind::Quality { sort: sort.clone() }.lte(kind),
            Self::Player => matches!(payload, Payload::Player { chosen: true }),
        }
    }
}
/// Discourse inspection of `ChoiceRef`.
pub trait ChoiceRefWordsExt: Sized {
    #[must_use]
    fn permits(&self, count: usize) -> bool;
}

impl ChoiceRefWordsExt for ChoiceRef {
    fn permits(&self, count: usize) -> bool {
        match self {
            Self::TheChoice => count == 1,
            Self::TheLatestChoice => count != 0,
        }
    }
}
impl DRS {
    #[must_use]
    pub fn count_outcomes(&self, sort: &OutcomeSort) -> usize {
        self.visible_referents().filter(|r| r.plur == Plurality::One && matches!(r.payload.as_ref(), Payload::Outcome { sort: found } if found == sort)).count()
    }
    #[must_use]
    pub fn count_choice(&self, sort: &ChoiceSort) -> usize {
        self.visible_referents()
            .filter(|r| r.plur == Plurality::One && sort.binds(&r.payload.kind(), &r.payload))
            .count()
    }
    #[must_use]
    pub fn count_ones(&self, kind: &Kind) -> usize {
        self.visible_referents()
            .filter(|r| {
                *r.payload != Payload::Difference
                    && r.plur == Plurality::One
                    && kind.lte(&r.payload.kind())
            })
            .count()
    }
    #[must_use]
    pub fn count_differences(&self) -> usize {
        self.visible_referents()
            .filter(|r| *r.payload == Payload::Difference && r.plur == Plurality::One)
            .count()
    }
    #[must_use]
    pub fn count_amount_outcomes(&self) -> usize {
        self.visible_referents()
            .filter(|r| {
                r.plur == Plurality::One
                    && matches!(r.payload.as_ref(), Payload::Outcome { sort } if sort.is_amount())
            })
            .count()
    }
}

/// Discourse inspection of `Kind`.
pub trait KindWordsExt: Sized {
    #[must_use]
    fn targetable(&self) -> bool;
    #[must_use]
    fn targeter(&self) -> bool;
    #[must_use]
    fn phrasal(&self) -> bool;
    #[must_use]
    fn binding(&self) -> Referent;
}

impl KindWordsExt for Kind {
    fn targetable(&self) -> bool {
        match self {
            Self::Object | Self::Player => true,
            Self::Join { left, right } => left.targetable() && right.targetable(),
            _ => false,
        }
    }
    fn targeter(&self) -> bool {
        match self {
            Self::Object => true,
            Self::Join { left, right } => left.targeter() && right.targeter(),
            _ => false,
        }
    }
    fn phrasal(&self) -> bool {
        match self {
            Self::Object | Self::Player | Self::Quality { .. } => true,
            Self::Join { left, right } => left.phrasal() && right.phrasal(),
            _ => false,
        }
    }
    fn binding(&self) -> Referent {
        Referent::new(
            Determiner::The,
            Plurality::One,
            if *self == Self::Player {
                Payload::Player { chosen: false }
            } else {
                Payload::Object {
                    ty: Vec::new(),
                    zone: None,
                    prov: None,
                    orig: None,
                    size: None,
                }
            },
        )
    }
}
