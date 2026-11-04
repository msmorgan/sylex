//! `conferral_rule` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::ConferralRule {
    type Target = sylex_core::ConferralRule;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::ConferralRule {
            scope: self.scope.lower(),
            confer: self.confer.lower(),
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
    fn lowers_conferral_rule() {
        assert_matches!(
            sylex_semantics_v1::ConferralRule {
                scope: minimal_predicate(),
                confer: minimal_property()
            }
            .lower(),
            sylex_core::ConferralRule {
                scope: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                confer: sylex_core::Property::Ability(_)
            }
        );
    }
}
