//! `color` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Color {
    type Target = sylex_core::Color;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::White => sylex_core::Color::White,
            Self::Blue => sylex_core::Color::Blue,
            Self::Black => sylex_core::Color::Black,
            Self::Red => sylex_core::Color::Red,
            Self::Green => sylex_core::Color::Green,
        }
    }
}

impl Lower for sylex_semantics_v1::ColorOrColorless {
    type Target = sylex_core::ColorOrColorless;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Colorless => sylex_core::ColorOrColorless::Colorless,
            Self::Color(f0) => sylex_core::ColorOrColorless::Color(f0.lower()),
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
    fn lowers_color_white() {
        assert_matches!(
            sylex_semantics_v1::Color::White.lower(),
            sylex_core::Color::White
        );
    }

    #[test]
    fn lowers_color_blue() {
        assert_matches!(
            sylex_semantics_v1::Color::Blue.lower(),
            sylex_core::Color::Blue
        );
    }

    #[test]
    fn lowers_color_black() {
        assert_matches!(
            sylex_semantics_v1::Color::Black.lower(),
            sylex_core::Color::Black
        );
    }

    #[test]
    fn lowers_color_red() {
        assert_matches!(
            sylex_semantics_v1::Color::Red.lower(),
            sylex_core::Color::Red
        );
    }

    #[test]
    fn lowers_color_green() {
        assert_matches!(
            sylex_semantics_v1::Color::Green.lower(),
            sylex_core::Color::Green
        );
    }

    #[test]
    fn lowers_color_or_colorless_colorless() {
        assert_matches!(
            sylex_semantics_v1::ColorOrColorless::Colorless.lower(),
            sylex_core::ColorOrColorless::Colorless
        );
    }

    #[test]
    fn lowers_color_or_colorless_color() {
        assert_matches!(
            sylex_semantics_v1::ColorOrColorless::Color(minimal_color()).lower(),
            sylex_core::ColorOrColorless::Color(sylex_core::Color::White)
        );
    }
}
