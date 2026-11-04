//! Declared lexical analysis and realization, independent of construction
//! parsing.

mod analysis;
mod model;
mod morphology;
pub mod numeral;
mod pronunciation;

pub use sylex_lexical_model::Case;
pub use sylex_lexical_model::Category;
pub use sylex_lexical_model::Countability;
pub use sylex_lexical_model::FeatureBundle;
pub use sylex_lexical_model::Finiteness;
pub use sylex_lexical_model::Frame;
pub use sylex_lexical_model::FrameItem;
pub use sylex_lexical_model::FrameSlot;
pub use sylex_lexical_model::LexemeId;
pub use sylex_lexical_model::LexicalReading;
pub use sylex_lexical_model::LexicalValue;
pub use sylex_lexical_model::Number;
pub use sylex_lexical_model::Numeral;
pub use sylex_lexical_model::Onset;
pub use sylex_lexical_model::Person;
pub use sylex_lexical_model::Relation;
pub use sylex_lexical_model::SurfaceCase;
pub use sylex_lexical_model::SurfaceFeatures;
pub use sylex_lexical_model::Tense;
pub use sylex_lexical_model::WordForm;

pub use crate::analysis::*;
pub use crate::model::*;
pub use crate::morphology::default_participle;
pub use crate::morphology::default_surface;
pub use crate::numeral::NumeralCodec;
pub use crate::numeral::ParseNumeralError;

mod registry;
pub use registry::{ClosedWord, LexicalClass, LexicalRegistry, PropertyDomain};
