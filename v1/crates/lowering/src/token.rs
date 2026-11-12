//! `token` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::TokenSpec {
    type Target = sylex_core::TokenSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Token(f0) => sylex_core::TokenSpec::Token(f0.lower()),
            Self::Named(f0) => sylex_core::TokenSpec::Named(f0.lower()),
            Self::Copy(f0) => sylex_core::TokenSpec::Copy(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::TokenName {
    type Target = sylex_core::TokenName;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::TokenName(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::PredefinedToken {
    type Target = sylex_core::PredefinedToken;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Treasure => sylex_core::PredefinedToken::Treasure,
            Self::Food => sylex_core::PredefinedToken::Food,
            Self::Gold => sylex_core::PredefinedToken::Gold,
            Self::Clue => sylex_core::PredefinedToken::Clue,
            Self::Blood => sylex_core::PredefinedToken::Blood,
            Self::Vibranium => sylex_core::PredefinedToken::Vibranium,
        }
    }
}

impl Lower for sylex_semantics_v1::Token {
    type Target = sylex_core::Token;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::Token {
            name: self.name.lower(),
            mana_cost: sylex_core::ManaCost::default(),
            color_indicator: self.color_indicator.lower(),
            supertypes: self.supertypes.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
            loyalty: None,
            defense: None,
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
    fn lowers_token_spec_token() {
        assert_matches!(
            sylex_semantics_v1::TokenSpec::Token(std::sync::Arc::new(minimal_token())).lower(),
            sylex_core::TokenSpec::Token(_)
        );
    }

    #[test]
    fn lowers_token_spec_named() {
        assert_matches!(
            sylex_semantics_v1::TokenSpec::Named(minimal_token_name()).lower(),
            sylex_core::TokenSpec::Named(sylex_core::TokenName(_))
        );
    }

    #[test]
    fn lowers_token_spec_copy() {
        assert_matches!(
            sylex_semantics_v1::TokenSpec::Copy(std::sync::Arc::new(minimal_copy_spec())).lower(),
            sylex_core::TokenSpec::Copy(_)
        );
    }

    #[test]
    fn lowers_token_name() {
        assert_matches!(
            sylex_semantics_v1::TokenName("X".into()).lower(),
            sylex_core::TokenName(_)
        );
    }

    #[test]
    fn lowers_predefined_token_treasure() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Treasure.lower(),
            sylex_core::PredefinedToken::Treasure
        );
    }

    #[test]
    fn lowers_predefined_token_food() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Food.lower(),
            sylex_core::PredefinedToken::Food
        );
    }

    #[test]
    fn lowers_predefined_token_gold() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Gold.lower(),
            sylex_core::PredefinedToken::Gold
        );
    }

    #[test]
    fn lowers_predefined_token_clue() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Clue.lower(),
            sylex_core::PredefinedToken::Clue
        );
    }

    #[test]
    fn lowers_predefined_token_blood() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Blood.lower(),
            sylex_core::PredefinedToken::Blood
        );
    }

    #[test]
    fn lowers_predefined_token_vibranium() {
        assert_matches!(
            sylex_semantics_v1::PredefinedToken::Vibranium.lower(),
            sylex_core::PredefinedToken::Vibranium
        );
    }

    #[test]
    fn lowers_token() {
        assert_matches!(
            sylex_semantics_v1::Token {
                name: None,
                color_indicator: [].into(),
                supertypes: [].into(),
                types: [].into(),
                subtypes: [].into(),
                abilities: [].into(),
                power: None,
                toughness: None
            }
            .lower(),
            sylex_core::Token {
                name: None,
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
