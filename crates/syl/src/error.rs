//! Lexing and parsing errors.

use crate::span::Span;

/// A lexing or parsing error, located by its span.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{kind}")]
pub struct Error {
    pub span: Span,
    pub kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }

    /// Formats the error as `path:line:column: message`.
    #[must_use]
    pub fn render(&self, path: &str, source: &str) -> String {
        format!("{path}:{}: {}", self.span.start_line_col(source), self.kind)
    }
}

/// What went wrong.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErrorKind {
    #[error("unexpected character {0:?}")]
    UnexpectedChar(char),
    #[error("'-' must begin an integer")]
    StrayMinus,
    #[error("'$' must be directly followed by a parameter name")]
    LoneDollar,
    #[error("words may not contain '.'")]
    DottedWord,
    #[error("invalid integer")]
    InvalidInteger,
    #[error("integers are written without leading zeros, and zero without '-'")]
    NonCanonicalInteger,
    #[error("integer out of range")]
    IntegerOutOfRange,
    #[error("unterminated string literal")]
    UnterminatedString,
    #[error("unsupported string escape {0:?}; only \\\" and \\\\ are allowed")]
    UnsupportedEscape(char),
    #[error("expected {expected}, found {found}")]
    Expected {
        expected: &'static str,
        found: String,
    },
    #[error("a parameter reference takes no arguments")]
    ParameterArguments,
    #[error("empty argument list: write a bare word instead")]
    EmptyCall,
    #[error("empty parameter list: omit the parentheses instead")]
    EmptyParameterList,
    #[error("empty sort argument list")]
    EmptySortArguments,
    #[error(
        "capitalized word `{0}`: only sorts and a primitive's constructor clause are \
         capitalized, outside `lexical` and `spelling`"
    )]
    CapitalizedWord(String),
    #[error("constructor `{0}` must be capitalized")]
    UncapitalizedConstructor(String),
}
