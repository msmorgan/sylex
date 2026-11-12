//! The data-driven syntax engine.
//!
//! A grammar data file of Schemas and Productions is bound to a lexical
//! environment (a [`sylex_lexicon::Lexicon`] and a [`FrameCatalog`]) into
//! a [`CompiledGrammar`]. Schemas compile Subcategorization Frames into
//! Productions, including their Valence Alternations. Parsing builds a
//! [`PackedForest`]; Readings are enumerated from it, checked by Admission
//! without parsing, and realized back to text.

mod alternatives;
mod chart;
mod compile;
mod counts;
mod fork_graph;
mod forks;
pub mod frames;
pub mod fug;
pub mod grammar;
mod hash;
mod laws;
mod normal_form;
mod prediction;
mod reading;
mod step;
#[cfg(feature = "tree-ranking-key")]
mod tree_key;
pub mod walk;

pub use alternatives::AlternativeDistribution;
pub use alternatives::Embedding;
pub use alternatives::EmbeddingStep;
pub use alternatives::HeadLeaf;
pub use alternatives::HeadWord;
pub use chart::ChildRef;
pub use chart::Metrics;
pub use chart::PackedForest;
pub use chart::Parse;
pub use compile::CompiledGrammar;
pub use compile::LoadReport;
pub use counts::Counts;
pub use counts::ForestCycle;
pub use forks::Answer;
pub use forks::Dependency;
pub use forks::Fork;
pub use forks::ForkForest;
pub use forks::ForkKind;
pub use forks::ForkOption;
pub use forks::ForkWord;
pub use forks::NarrowError;
pub use frames::FrameCatalog;
pub use grammar::Grammar;
pub use laws::RealizationFault;
pub use reading::AdmissionError;
pub use reading::Constituent;
pub use reading::RankingKey;
pub use reading::Reading;
pub use reading::ReadingNode;
pub use step::Gap;

/// Why a grammar or frame catalog cannot be loaded or bound.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LoadError {
    #[error("malformed data file: {0}")]
    Format(String),
    #[error("{0}")]
    Invalid(String),
}
