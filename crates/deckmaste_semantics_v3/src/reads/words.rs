//! Scalar and payload facts from `Semantics/Check/Words.lean`.
use super::DRS;
use super::Determiner;
use super::Origin;
use super::Payload;
use super::Referent;
use super::Stamp;
use crate::ast::CardType;
use crate::ast::ChoiceRef;
use crate::ast::ChoiceSort;
use crate::ast::Delta;
use crate::ast::Kind;
use crate::ast::OutcomeSort;
use crate::ast::PileFace;
use crate::ast::Plurality;
use crate::ast::QualitySort;
use crate::ast::Stat;
use crate::ast::Subtype;

impl CardType {
    #[must_use]
    pub fn creature() -> Self {
        Self {
            name: "Creature".into(),
            permanent: true,
            stats: vec![Stat::Power, Stat::Toughness],
            instant_or_sorcery: false,
        }
    }
    #[must_use]
    pub fn artifact() -> Self {
        Self::plain("Artifact", true, false)
    }
    #[must_use]
    pub fn land() -> Self {
        Self::plain("Land", true, false)
    }
    #[must_use]
    pub fn enchantment() -> Self {
        Self::plain("Enchantment", true, false)
    }
    #[must_use]
    pub fn instant() -> Self {
        Self::plain("Instant", false, true)
    }
    #[must_use]
    pub fn sorcery() -> Self {
        Self::plain("Sorcery", false, true)
    }
    #[must_use]
    pub fn planeswalker() -> Self {
        Self {
            stats: vec![Stat::Loyalty],
            ..Self::plain("Planeswalker", true, false)
        }
    }
    #[must_use]
    pub fn battle() -> Self {
        Self {
            stats: vec![Stat::Defense],
            ..Self::plain("Battle", true, false)
        }
    }
    #[must_use]
    pub fn kindred() -> Self {
        Self::plain("Kindred", false, false)
    }
    #[must_use]
    pub fn dungeon() -> Self {
        Self::plain("Dungeon", false, false)
    }
    fn plain(name: &str, permanent: bool, instant_or_sorcery: bool) -> Self {
        Self {
            name: name.into(),
            permanent,
            stats: vec![],
            instant_or_sorcery,
        }
    }
    #[must_use]
    pub fn all() -> [Self; 10] {
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
impl QualitySort {
    #[must_use]
    pub fn chosen_read_ok(&self) -> bool {
        !matches!(self, Self::Number | Self::CounterKind)
    }
}
impl OutcomeSort {
    #[must_use]
    pub fn comparable(&self) -> bool {
        matches!(self, Self::RollResult | Self::DamageMarked)
    }
    #[must_use]
    pub fn is_amount(&self) -> bool {
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
impl<T> Delta<T> {
    pub fn amount(&self) -> &T {
        match self {
            Self::Up { amount } | Self::Down { amount } | Self::Set { amount } => amount,
        }
    }
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set { .. })
    }
}
impl Subtype {
    #[must_use]
    pub fn card_type(&self) -> Option<&CardType> {
        match self {
            Self::Of { host, .. } => Some(host),
            Self::Spell { .. } => None,
        }
    }
    #[must_use]
    pub fn fits(&self, ty: &CardType) -> bool {
        match self {
            Self::Of { host, .. } => host == ty,
            Self::Spell { .. } => ty.instant_or_sorcery,
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
impl ChoiceSort {
    #[must_use]
    pub fn at(kind: &Kind) -> Option<Self> {
        match kind {
            Kind::Quality { sort } => Some(Self::Quality { sort: sort.clone() }),
            Kind::Player => Some(Self::Player),
            _ => None,
        }
    }
    #[must_use]
    pub fn binding(&self) -> Referent {
        let payload = match self {
            Self::Quality { sort } => Payload::Quality { q: sort.clone() },
            Self::Player => Payload::Player { chosen: true },
        };
        Referent::new(Determiner::A, Plurality::One, payload)
    }
    #[must_use]
    pub fn binds(&self, kind: &Kind, payload: &Payload) -> bool {
        match self {
            Self::Quality { sort } => Kind::Quality { sort: sort.clone() }.lte(kind),
            Self::Player => matches!(payload, Payload::Player { chosen: true }),
        }
    }
}
impl ChoiceRef {
    #[must_use]
    pub fn permits(&self, count: usize) -> bool {
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
