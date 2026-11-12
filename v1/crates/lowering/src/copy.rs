//! `copy` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::CopySpec {
    type Target = sylex_core::CopySpec;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::CopySpec {
            source: self.source.lower(),
            exceptions: self.exceptions.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::CopySource {
    type Target = sylex_core::CopySource;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object(f0) => sylex_core::CopySource::Object(f0.lower()),
            Self::SelfCard => sylex_core::CopySource::SelfCard,
        }
    }
}

impl Lower for sylex_semantics_v1::CopyException {
    type Target = sylex_core::CopyException;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0) => sylex_core::CopyException::Modify(f0.lower()),
            Self::Retain(f0) => sylex_core::CopyException::Retain(f0.lower()),
            Self::AdditionalEffect(f0) => sylex_core::CopyException::AdditionalEffect(f0.lower()),
            Self::NoManaCost => sylex_core::CopyException::NoManaCost,
        }
    }
}

impl Lower for sylex_semantics_v1::CopiableValues {
    type Target = sylex_core::CopiableValues;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::CopiableValues {
            name: self.name.lower(),
            mana_cost: self.mana_cost.lower(),
            color_indicator: self.color_indicator.lower(),
            supertypes: self.supertypes.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
            loyalty: self.loyalty.lower(),
            defense: self.defense.lower(),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_copy_spec() {
        assert_matches!(
            sylex_semantics_v1::CopySpec {
                source: minimal_copy_source(),
                exceptions: Vec::new()
            }
            .lower(),
            sylex_core::CopySpec {
                source: sylex_core::CopySource::Object(sylex_core::Reference::Reg(
                    sylex_core::RefId(0)
                )),
                exceptions: _
            }
        );
    }

    #[test]
    fn lowers_copy_source_object() {
        assert_matches!(
            sylex_semantics_v1::CopySource::Object(minimal_reference()).lower(),
            sylex_core::CopySource::Object(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_copy_source_self_card() {
        assert_matches!(
            sylex_semantics_v1::CopySource::SelfCard.lower(),
            sylex_core::CopySource::SelfCard
        );
    }

    #[test]
    fn lowers_copy_exception_modify() {
        assert_matches!(
            sylex_semantics_v1::CopyException::Modify(minimal_modification()).lower(),
            sylex_core::CopyException::Modify(sylex_core::Modification::Power(
                sylex_core::NumericOp::Set(sylex_core::StatValue::DefinedByAbility)
            ))
        );
    }

    #[test]
    fn lowers_copy_exception_retain() {
        assert_matches!(
            sylex_semantics_v1::CopyException::Retain(minimal_characteristic()).lower(),
            sylex_core::CopyException::Retain(sylex_core::Characteristic::Colors)
        );
    }

    #[test]
    fn lowers_copy_exception_additional_effect() {
        assert_matches!(
            sylex_semantics_v1::CopyException::AdditionalEffect(minimal_enter_rider()).lower(),
            sylex_core::CopyException::AdditionalEffect(sylex_core::EnterRider::Tapped)
        );
    }

    #[test]
    fn lowers_copy_exception_no_mana_cost() {
        assert_matches!(
            sylex_semantics_v1::CopyException::NoManaCost.lower(),
            sylex_core::CopyException::NoManaCost
        );
    }

    #[test]
    fn lowers_copiable_values() {
        assert_matches!(
            sylex_semantics_v1::CopiableValues {
                name: "x".into(),
                mana_cost: sylex_semantics_v1::ManaCost::from(std::sync::Arc::<
                    [sylex_semantics_v1::ManaSymbol],
                >::from([])),
                color_indicator: Vec::new(),
                supertypes: Vec::new(),
                types: Vec::new(),
                subtypes: Vec::new(),
                abilities: Vec::new(),
                power: None,
                toughness: None,
                loyalty: None,
                defense: None
            }
            .lower(),
            sylex_core::CopiableValues {
                name: _,
                mana_cost: _,
                color_indicator: _,
                supertypes: _,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None,
                loyalty: None,
                defense: None
            }
        );
    }
}
