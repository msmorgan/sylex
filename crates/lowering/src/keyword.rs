//! `keyword` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::KeywordRef {
    type Target = sylex_core::KeywordRef;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::KeywordRef(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::ParamShape {
    type Target = sylex_core::ParamShape;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => sylex_core::ParamShape::None,
            Self::Counted => sylex_core::ParamShape::Counted,
            Self::Costed => sylex_core::ParamShape::Costed,
            Self::CountedCost => sylex_core::ParamShape::CountedCost,
            Self::Predicated => sylex_core::ParamShape::Predicated,
            Self::PredicatedCosted => sylex_core::ParamShape::PredicatedCosted,
            Self::Named => sylex_core::ParamShape::Named,
        }
    }
}

impl Lower for sylex_semantics_v1::KeywordDecl {
    type Target = sylex_core::KeywordDecl;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::KeywordDecl {
            name: self.name.lower(),
            shape: self.shape.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::KeywordAbility {
    type Target = sylex_core::KeywordAbility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FirstStrike => sylex_core::KeywordAbility::FirstStrike,
            Self::DoubleStrike => sylex_core::KeywordAbility::DoubleStrike,
            Self::Deathtouch => sylex_core::KeywordAbility::Deathtouch,
            Self::Trample => sylex_core::KeywordAbility::Trample,
            Self::Vigilance => sylex_core::KeywordAbility::Vigilance,
            Self::Composite { name, abilities } => sylex_core::KeywordAbility::Composite {
                name: name.lower(),
                abilities: abilities.lower(),
            },
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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
    fn lowers_keyword_ref() {
        assert_matches!(
            sylex_semantics_v1::KeywordRef("X".into()).lower(),
            sylex_core::KeywordRef(_)
        );
    }

    #[test]
    fn lowers_param_shape_none() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::None.lower(),
            sylex_core::ParamShape::None
        );
    }

    #[test]
    fn lowers_param_shape_counted() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::Counted.lower(),
            sylex_core::ParamShape::Counted
        );
    }

    #[test]
    fn lowers_param_shape_costed() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::Costed.lower(),
            sylex_core::ParamShape::Costed
        );
    }

    #[test]
    fn lowers_param_shape_counted_cost() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::CountedCost.lower(),
            sylex_core::ParamShape::CountedCost
        );
    }

    #[test]
    fn lowers_param_shape_predicated() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::Predicated.lower(),
            sylex_core::ParamShape::Predicated
        );
    }

    #[test]
    fn lowers_param_shape_predicated_costed() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::PredicatedCosted.lower(),
            sylex_core::ParamShape::PredicatedCosted
        );
    }

    #[test]
    fn lowers_param_shape_named() {
        assert_matches!(
            sylex_semantics_v1::ParamShape::Named.lower(),
            sylex_core::ParamShape::Named
        );
    }

    #[test]
    fn lowers_keyword_decl() {
        assert_matches!(
            sylex_semantics_v1::KeywordDecl {
                name: "X".into(),
                shape: minimal_param_shape()
            }
            .lower(),
            sylex_core::KeywordDecl {
                name: _,
                shape: sylex_core::ParamShape::None
            }
        );
    }

    #[test]
    fn lowers_keyword_ability_first_strike() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::FirstStrike.lower(),
            sylex_core::KeywordAbility::FirstStrike
        );
    }

    #[test]
    fn lowers_keyword_ability_double_strike() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::DoubleStrike.lower(),
            sylex_core::KeywordAbility::DoubleStrike
        );
    }

    #[test]
    fn lowers_keyword_ability_deathtouch() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::Deathtouch.lower(),
            sylex_core::KeywordAbility::Deathtouch
        );
    }

    #[test]
    fn lowers_keyword_ability_trample() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::Trample.lower(),
            sylex_core::KeywordAbility::Trample
        );
    }

    #[test]
    fn lowers_keyword_ability_vigilance() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::Vigilance.lower(),
            sylex_core::KeywordAbility::Vigilance
        );
    }

    #[test]
    fn lowers_keyword_ability_composite() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::Composite {
                name: "X".into(),
                abilities: Vec::new()
            }
            .lower(),
            sylex_core::KeywordAbility::Composite {
                name: _,
                abilities: _
            }
        );
    }

    #[test]
    fn lowers_keyword_ability_expanded() {
        assert_matches!(
            sylex_semantics_v1::KeywordAbility::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_keyword_ability())
            })
            .lower(),
            sylex_core::KeywordAbility::FirstStrike
        );
    }
}
