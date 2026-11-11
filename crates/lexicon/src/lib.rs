//! Declared lexical analysis and realization, independent of construction
//! parsing.

mod analysis;
mod dimensions;
mod model;
mod morphology;
pub mod numeral;
mod pronunciation;
mod scope;
mod types;

pub use crate::analysis::*;
pub use crate::dimensions::AgreementProperties;
pub use crate::dimensions::Edges;
pub use crate::dimensions::FeatureBundle;
pub use crate::dimensions::GenitiveAlternant;
pub use crate::dimensions::Hyphenation;
pub use crate::dimensions::LeftEdge;
pub use crate::dimensions::NameForm;
pub use crate::dimensions::Onset;
pub use crate::dimensions::RightEdge;
pub use crate::model::*;
pub use crate::morphology::default_participle;
pub use crate::morphology::default_surface;
pub use crate::numeral::NumeralCodec;
pub use crate::numeral::ParseNumeralError;
pub use crate::pronunciation::numeral_onset;
pub use crate::scope::CardScope;
pub use crate::types::Case;
pub use crate::types::Category;
pub use crate::types::Countability;
pub use crate::types::Finiteness;
pub use crate::types::LexemeId;
pub use crate::types::LexicalReading;
pub use crate::types::LexicalValue;
pub use crate::types::Number;
pub use crate::types::Numeral;
pub use crate::types::Person;
pub use crate::types::SurfaceCase;
pub use crate::types::SurfaceFeatures;
pub use crate::types::Tense;
pub use crate::types::WordForm;

mod registry;
pub use registry::ClosedWord;
pub use registry::LexicalClass;
pub use registry::LexicalRegistry;
pub use registry::PropertyDomain;
