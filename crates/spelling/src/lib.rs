//! The spelling layer (`docs/design/spelling.md`).
//!
//! [`notation`] parses a `.syl` declaration's `spelling` string according to
//! the notation grammar of `docs/design/spelling.md` §7.

pub mod notation;

pub use notation::Affix;
pub use notation::Error;
pub use notation::ErrorKind;
pub use notation::Form;
pub use notation::Hole;
pub use notation::HoleKind;
pub use notation::Join;
pub use notation::Mark;
pub use notation::Optional;
pub use notation::OwnName;
pub use notation::Part;
pub use notation::Span;
pub use notation::Spelling;
pub use notation::Tail;
pub use notation::TailPlacement;
pub use notation::Text;
pub use notation::parse;
