//! Parses `.syl` source files to a purely syntactic tree.
//!
//! The `.syl` language is described in `docs/decisions/semantics-v3.md` §4.4.
//! This crate only reads its syntax: it does not resolve names, expand
//! templates, check sorts or treat any declaration kind specially. A
//! [`File`] is a sequence of [`Declaration`]s and [`Comment`]s; every node
//! carries a [`Span`].
//!
//! Lexing ([`lex()`]) is separate from building the tree ([`parse()`]).

mod error;
pub mod lex;
mod parse;
mod span;
mod tree;

pub use error::Error;
pub use error::ErrorKind;
pub use lex::lex;
pub use parse::parse;
pub use span::FileId;
pub use span::LineCol;
pub use span::Span;
pub use tree::Arg;
pub use tree::Call;
pub use tree::Comment;
pub use tree::Declaration;
pub use tree::Entry;
pub use tree::Field;
pub use tree::File;
pub use tree::Item;
pub use tree::Name;
pub use tree::NamedShape;
pub use tree::Param;
pub use tree::Record;
pub use tree::Shape;
pub use tree::Sort;
pub use tree::Value;
pub use tree::ValueKind;
