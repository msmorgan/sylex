//! Parses `.syl` source files to a purely syntactic tree.
//!
//! The `.syl` language is described in `docs/decisions/syl.md` §2.
//! This crate only reads its syntax: it does not resolve names, expand
//! templates, check sorts or treat any declaration kind specially. A
//! [`File`] is a sequence of [`Declaration`]s and [`Comment`]s; every node
//! carries a [`Span`].
//!
//! Lexing ([`lex()`]) is separate from building the tree ([`parse()`]).
//! [`format()`] prints a tree back in a canonical layout, and
//! [`same_meaning`] checks that a reformatted file still means the same.
//! [`bundle`] loads a plugin directory's files, parsed once, for later layers.
//! [`cli`] is the formatter's command line, run by the `sylfmt` binary.

pub mod bundle;
pub mod cli;
mod error;
mod format;
pub mod lex;
mod meaning;
mod parse;
mod span;
mod tree;

pub use bundle::Bundle;
pub use bundle::SourceFile;
pub use error::Error;
pub use error::ErrorKind;
pub use format::Closers;
pub use format::Style;
pub use format::TrailingCommas;
pub use format::format;
pub use lex::lex;
pub use meaning::same_meaning;
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
