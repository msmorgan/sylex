//! `status` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Status {
    type Target = sylex_core::Status;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => sylex_core::Status::Tapped,
            Self::Untapped => sylex_core::Status::Untapped,
            Self::Flipped => sylex_core::Status::Flipped,
            Self::Unflipped => sylex_core::Status::Unflipped,
            Self::FaceDown => sylex_core::Status::FaceDown,
            Self::FaceUp => sylex_core::Status::FaceUp,
            Self::PhasedOut => sylex_core::Status::PhasedOut,
            Self::PhasedIn => sylex_core::Status::PhasedIn,
        }
    }
}

impl Lower for sylex_semantics_v1::Face {
    type Target = sylex_core::Face;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Up => sylex_core::Face::Up,
            Self::Down => sylex_core::Face::Down,
        }
    }
}

impl Lower for sylex_semantics_v1::Phasing {
    type Target = sylex_core::Phasing;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::In => sylex_core::Phasing::In,
            Self::Out => sylex_core::Phasing::Out,
        }
    }
}

impl Lower for sylex_semantics_v1::FaceDownSpec {
    type Target = sylex_core::FaceDownSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Listed(f0) => sylex_core::FaceDownSpec::Listed(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::FaceDownCharacteristics {
    type Target = sylex_core::FaceDownCharacteristics;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::FaceDownCharacteristics {
            name: self.name.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
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
    fn lowers_status_tapped() {
        assert_matches!(
            sylex_semantics_v1::Status::Tapped.lower(),
            sylex_core::Status::Tapped
        );
    }

    #[test]
    fn lowers_status_untapped() {
        assert_matches!(
            sylex_semantics_v1::Status::Untapped.lower(),
            sylex_core::Status::Untapped
        );
    }

    #[test]
    fn lowers_status_flipped() {
        assert_matches!(
            sylex_semantics_v1::Status::Flipped.lower(),
            sylex_core::Status::Flipped
        );
    }

    #[test]
    fn lowers_status_unflipped() {
        assert_matches!(
            sylex_semantics_v1::Status::Unflipped.lower(),
            sylex_core::Status::Unflipped
        );
    }

    #[test]
    fn lowers_status_face_down() {
        assert_matches!(
            sylex_semantics_v1::Status::FaceDown.lower(),
            sylex_core::Status::FaceDown
        );
    }

    #[test]
    fn lowers_status_face_up() {
        assert_matches!(
            sylex_semantics_v1::Status::FaceUp.lower(),
            sylex_core::Status::FaceUp
        );
    }

    #[test]
    fn lowers_status_phased_out() {
        assert_matches!(
            sylex_semantics_v1::Status::PhasedOut.lower(),
            sylex_core::Status::PhasedOut
        );
    }

    #[test]
    fn lowers_status_phased_in() {
        assert_matches!(
            sylex_semantics_v1::Status::PhasedIn.lower(),
            sylex_core::Status::PhasedIn
        );
    }

    #[test]
    fn lowers_face_up() {
        assert_matches!(sylex_semantics_v1::Face::Up.lower(), sylex_core::Face::Up);
    }

    #[test]
    fn lowers_face_down() {
        assert_matches!(
            sylex_semantics_v1::Face::Down.lower(),
            sylex_core::Face::Down
        );
    }

    #[test]
    fn lowers_phasing_in() {
        assert_matches!(
            sylex_semantics_v1::Phasing::In.lower(),
            sylex_core::Phasing::In
        );
    }

    #[test]
    fn lowers_phasing_out() {
        assert_matches!(
            sylex_semantics_v1::Phasing::Out.lower(),
            sylex_core::Phasing::Out
        );
    }

    #[test]
    fn lowers_face_down_spec_listed() {
        assert_matches!(
            sylex_semantics_v1::FaceDownSpec::Listed(minimal_face_down_characteristics()).lower(),
            sylex_core::FaceDownSpec::Listed(sylex_core::FaceDownCharacteristics {
                name: None,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None
            })
        );
    }

    #[test]
    fn lowers_face_down_characteristics() {
        assert_matches!(
            sylex_semantics_v1::FaceDownCharacteristics {
                name: None,
                types: Vec::new(),
                subtypes: Vec::new(),
                abilities: Vec::new(),
                power: None,
                toughness: None
            }
            .lower(),
            sylex_core::FaceDownCharacteristics {
                name: None,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None
            }
        );
    }
}
