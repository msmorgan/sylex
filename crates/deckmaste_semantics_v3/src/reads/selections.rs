//! Selection and creation metadata from the corresponding Lean checker modules.
use super::Determiner;
use super::Origin;
use super::Payload;
use super::Referent;
use super::merge_types;
use super::normalize_types;
use crate::ast::Amount;
use crate::ast::CardType;
use crate::ast::CreationSpec;
use crate::ast::Plurality;
use crate::ast::Quantity;
use crate::ast::SliceCount;
use crate::ast::TokenSpec;
use crate::ast::Zone;
impl Amount {
    #[must_use]
    pub fn exact(&self) -> Option<u64> {
        self.exact_at(64)
    }
    fn exact_at(&self, fuel: u64) -> Option<u64> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.exact_at(fuel - 1),
            (1.., Self::Lit { value }) => (*value).try_into().ok(),
            _ => None,
        }
    }
}
impl Quantity {
    #[must_use]
    pub fn exact(&self) -> Option<u64> {
        self.exact_at(64)
    }
    fn exact_at(&self, fuel: u64) -> Option<u64> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => body.exact_at(fuel - 1),
            (1.., Self::ExactlyOf { amount }) => amount.exact_at(fuel - 1),
            (
                1..,
                Self::Range {
                    low: Some(low),
                    high: Some(high),
                },
            ) if low == high => Some(*low),
            _ => None,
        }
    }
}
impl SliceCount {
    #[must_use]
    pub fn exact(&self) -> Option<u64> {
        match self {
            Self::Counted { quantity } => quantity.exact(),
            Self::Whole | Self::Totalling { .. } => None,
        }
    }
}
pub fn selection_payload(
    types: &[CardType],
    group: Option<&Referent>,
    count: &SliceCount,
) -> Payload {
    if let Some(Payload::Player { chosen }) = group.map(|r| r.payload.as_ref()) {
        return Payload::Player { chosen: *chosen };
    }
    Payload::Object {
        ty: merge_types(&group.map_or_else(Vec::new, |r| r.payload.types()), types),
        zone: group.and_then(|r| r.payload.zone().cloned()),
        prov: None,
        orig: None,
        size: count.exact(),
    }
}
#[must_use]
pub fn slice_payload(zone: Option<Zone>, amount: &Amount) -> Payload {
    Payload::Object {
        ty: vec![],
        zone,
        prov: None,
        orig: None,
        size: amount.exact(),
    }
}
#[must_use]
pub fn run_payloads(zone: Option<Zone>, stop_types: Vec<CardType>) -> (Payload, Payload) {
    (
        Payload::Object {
            ty: vec![],
            zone: zone.clone(),
            prov: None,
            orig: None,
            size: None,
        },
        Payload::Object {
            ty: stop_types,
            zone,
            prov: None,
            orig: None,
            size: Some(1),
        },
    )
}
impl TokenSpec {
    fn created_types(&self, source_types: &[CardType], fuel: u64) -> Vec<CardType> {
        match (fuel, self) {
            (0, _) => vec![],
            (_, Self::InCaller { body }) => body.created_types(source_types, fuel - 1),
            (_, Self::Written { characteristics }) => {
                normalize_types(&characteristics.characteristics.types)
            }
            (_, Self::DoubleFaced { front, .. }) => normalize_types(&front.characteristics.types),
            (_, Self::AsThose | Self::CopyOf { .. }) => normalize_types(source_types),
        }
    }
}
impl CreationSpec {
    #[must_use]
    pub fn created_referent(
        &self,
        count: &Plurality,
        performer: &Plurality,
        source_types: &[CardType],
    ) -> Referent {
        let (ty, zone, orig) = match self {
            Self::Token { spec, .. } => (
                spec.created_types(source_types, 64),
                Zone::Battlefield,
                Some(Origin::Token),
            ),
            Self::Emblem { .. } => (vec![], Zone::Command, None),
        };
        let plur = if *count == Plurality::One && *performer == Plurality::One {
            Plurality::One
        } else {
            Plurality::Many
        };
        let mut referent = Referent::new(
            Determiner::A,
            plur,
            Payload::Object {
                ty,
                zone: Some(zone),
                prov: None,
                orig,
                size: None,
            },
        );
        referent.created_result = true;
        referent
    }
}
