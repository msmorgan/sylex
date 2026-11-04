//! `counter` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::CounterRef {
    type Target = sylex_core::CounterRef;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::CounterRef(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::CounterSpec {
    type Target = sylex_core::CounterSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Named(f0, f1) => sylex_core::CounterSpec::Named(f0.lower(), f1.lower()),
            Self::AllKinds => sylex_core::CounterSpec::AllKinds,
        }
    }
}

impl Lower for sylex_semantics_v1::CounterScope {
    type Target = sylex_core::CounterScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => sylex_core::CounterScope::Object,
            Self::Player => sylex_core::CounterScope::Player,
        }
    }
}

impl Lower for sylex_semantics_v1::Counter {
    type Target = sylex_core::Counter;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::Counter {
            name: self.name.lower(),
            scope: self.scope.lower(),
            confers: self.confers.lower(),
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
    fn lowers_counter_ref() {
        assert_matches!(
            sylex_semantics_v1::CounterRef("X".into()).lower(),
            sylex_core::CounterRef(_)
        );
    }

    #[test]
    fn lowers_counter_spec_named() {
        assert_matches!(
            sylex_semantics_v1::CounterSpec::Named(minimal_counter_ref(), minimal_count()).lower(),
            sylex_core::CounterSpec::Named(
                sylex_core::CounterRef(_),
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_counter_spec_all_kinds() {
        assert_matches!(
            sylex_semantics_v1::CounterSpec::AllKinds.lower(),
            sylex_core::CounterSpec::AllKinds
        );
    }

    #[test]
    fn lowers_counter_scope_object() {
        assert_matches!(
            sylex_semantics_v1::CounterScope::Object.lower(),
            sylex_core::CounterScope::Object
        );
    }

    #[test]
    fn lowers_counter_scope_player() {
        assert_matches!(
            sylex_semantics_v1::CounterScope::Player.lower(),
            sylex_core::CounterScope::Player
        );
    }

    #[test]
    fn lowers_counter() {
        assert_matches!(
            sylex_semantics_v1::Counter {
                name: "X".into(),
                scope: minimal_counter_scope(),
                confers: Vec::new()
            }
            .lower(),
            sylex_core::Counter {
                name: _,
                scope: sylex_core::CounterScope::Object,
                confers: _
            }
        );
    }
}
