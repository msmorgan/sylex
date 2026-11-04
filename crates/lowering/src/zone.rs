//! `zone` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Zone {
    type Target = sylex_core::Zone;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Battlefield => sylex_core::Zone::Battlefield,
            Self::Command => sylex_core::Zone::Command,
            Self::Exile => sylex_core::Zone::Exile,
            Self::Graveyard => sylex_core::Zone::Graveyard,
            Self::Hand => sylex_core::Zone::Hand,
            Self::Library => sylex_core::Zone::Library,
            Self::Stack => sylex_core::Zone::Stack,
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
    fn lowers_zone_battlefield() {
        assert_matches!(
            sylex_semantics_v1::Zone::Battlefield.lower(),
            sylex_core::Zone::Battlefield
        );
    }

    #[test]
    fn lowers_zone_command() {
        assert_matches!(
            sylex_semantics_v1::Zone::Command.lower(),
            sylex_core::Zone::Command
        );
    }

    #[test]
    fn lowers_zone_exile() {
        assert_matches!(
            sylex_semantics_v1::Zone::Exile.lower(),
            sylex_core::Zone::Exile
        );
    }

    #[test]
    fn lowers_zone_graveyard() {
        assert_matches!(
            sylex_semantics_v1::Zone::Graveyard.lower(),
            sylex_core::Zone::Graveyard
        );
    }

    #[test]
    fn lowers_zone_hand() {
        assert_matches!(
            sylex_semantics_v1::Zone::Hand.lower(),
            sylex_core::Zone::Hand
        );
    }

    #[test]
    fn lowers_zone_library() {
        assert_matches!(
            sylex_semantics_v1::Zone::Library.lower(),
            sylex_core::Zone::Library
        );
    }

    #[test]
    fn lowers_zone_stack() {
        assert_matches!(
            sylex_semantics_v1::Zone::Stack.lower(),
            sylex_core::Zone::Stack
        );
    }
}
