//! `damage_result_rule` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::DamageResultRule {
    type Target = sylex_core::DamageResultRule;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::DamageResultRule {
            recipient: self.recipient.lower(),
            remove: self.remove.lower(),
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
    fn lowers_damage_result_rule() {
        assert_matches!(
            sylex_semantics_v1::DamageResultRule {
                recipient: minimal_predicate(),
                remove: minimal_counter_ref()
            }
            .lower(),
            sylex_core::DamageResultRule {
                recipient: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                ),
                remove: sylex_core::CounterRef(_)
            }
        );
    }
}
