//! Splits `.syl` source into tokens.
//!
//! Whitespace is dropped; comments are kept as tokens. Each token records
//! whether a blank line separates it from the token before it.

use dpsi::Ident;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::span::FileId;
use crate::span::Span;

/// One lexical token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    /// Whether a blank line lies between this token and the previous one (or
    /// the start of the file).
    pub blank_line_before: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// `[A-Za-z_][A-Za-z0-9_]*`.
    Word(Ident),
    /// `.word`, naming a constructor; the dot is not part of the ident.
    Constructor(Ident),
    /// A double-quoted string, unescaped.
    String(String),
    /// `-?[0-9]+`, written without leading zeros and never as `-0`.
    Integer(i64),
    /// `#` to the end of the line; the text excludes the `#`.
    Comment(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Equals,
    /// The end of the file; its span is empty.
    Eof,
}

impl TokenKind {
    /// A short description for error messages.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            TokenKind::Word(w) => format!("word `{w}`"),
            TokenKind::Constructor(w) => format!("constructor `.{w}`"),
            TokenKind::String(_) => "a string".to_owned(),
            TokenKind::Integer(n) => format!("integer `{n}`"),
            TokenKind::Comment(_) => "a comment".to_owned(),
            TokenKind::LParen => "'('".to_owned(),
            TokenKind::RParen => "')'".to_owned(),
            TokenKind::LBracket => "'['".to_owned(),
            TokenKind::RBracket => "']'".to_owned(),
            TokenKind::LBrace => "'{'".to_owned(),
            TokenKind::RBrace => "'}'".to_owned(),
            TokenKind::Comma => "','".to_owned(),
            TokenKind::Colon => "':'".to_owned(),
            TokenKind::Equals => "'='".to_owned(),
            TokenKind::Eof => "end of file".to_owned(),
        }
    }
}

fn is_word_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_word_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Splits `source` into tokens, ending with [`TokenKind::Eof`].
///
/// # Errors
///
/// On a character no token can start with, a `-` not followed by a digit, a
/// word containing `.`, an integer that is malformed, non-canonical or out of
/// `i64` range, an unterminated string, or an escape other than `\"` and `\\`.
pub fn lex(file: FileId, source: &str) -> Result<Vec<Token>, Error> {
    let bytes = source.as_bytes();
    let span = |start: usize, end: usize| Span::new(file, start, end);
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut newlines = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'\n' => {
                newlines += 1;
                i += 1;
                continue;
            }
            b' ' | b'\t' | b'\r' => {
                i += 1;
                continue;
            }
            _ => {}
        }
        // At the start of the file a single newline already ends a blank line.
        let blank_line_before = if tokens.is_empty() { newlines >= 1 } else { newlines >= 2 };
        newlines = 0;
        let start = i;
        let kind = match b {
            b'#' => {
                let mut end = source[i..].find('\n').map_or(bytes.len(), |n| i + n);
                // A `\r` before the newline belongs to the line ending.
                if source[..end].ends_with('\r') {
                    end -= 1;
                }
                let text = &source[i + 1..end];
                i = end;
                TokenKind::Comment(text.to_owned())
            }
            b'"' => {
                i += 1;
                let mut text = String::new();
                loop {
                    let Some(c) = source[i..].chars().next() else {
                        return Err(Error::new(span(start, i), ErrorKind::UnterminatedString));
                    };
                    match c {
                        '"' => {
                            i += 1;
                            break;
                        }
                        '\\' => {
                            let Some(escaped) = source[i + 1..].chars().next() else {
                                return Err(Error::new(
                                    span(start, i + 1),
                                    ErrorKind::UnterminatedString,
                                ));
                            };
                            if escaped != '"' && escaped != '\\' {
                                return Err(Error::new(
                                    span(i, i + 1 + escaped.len_utf8()),
                                    ErrorKind::UnsupportedEscape(escaped),
                                ));
                            }
                            text.push(escaped);
                            i += 2;
                        }
                        c => {
                            text.push(c);
                            i += c.len_utf8();
                        }
                    }
                }
                TokenKind::String(text)
            }
            b'(' | b')' | b'[' | b']' | b'{' | b'}' | b',' | b':' | b'=' => {
                i += 1;
                match b {
                    b'(' => TokenKind::LParen,
                    b')' => TokenKind::RParen,
                    b'[' => TokenKind::LBracket,
                    b']' => TokenKind::RBracket,
                    b'{' => TokenKind::LBrace,
                    b'}' => TokenKind::RBrace,
                    b',' => TokenKind::Comma,
                    b':' => TokenKind::Colon,
                    _ => TokenKind::Equals,
                }
            }
            b'-' | b'0'..=b'9' => {
                let (kind, end) = integer(source, start, span)?;
                i = end;
                kind
            }
            b'.' if bytes.get(i + 1).copied().is_some_and(is_word_start) => {
                i += 1;
                let word_start = i;
                while bytes.get(i).copied().is_some_and(is_word_continue) {
                    i += 1;
                }
                if bytes.get(i) == Some(&b'.') {
                    return Err(dotted_word(bytes, start, i, span));
                }
                TokenKind::Constructor(Ident::new(&source[word_start..i]))
            }
            b if is_word_start(b) => {
                while bytes.get(i).copied().is_some_and(is_word_continue) {
                    i += 1;
                }
                if bytes.get(i) == Some(&b'.') {
                    return Err(dotted_word(bytes, start, i, span));
                }
                TokenKind::Word(Ident::new(&source[start..i]))
            }
            _ => {
                let c = source[i..].chars().next().unwrap_or_default();
                return Err(Error::new(
                    span(start, i + c.len_utf8()),
                    ErrorKind::UnexpectedChar(c),
                ));
            }
        };
        tokens.push(Token {
            kind,
            span: span(start, i),
            blank_line_before,
        });
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: span(bytes.len(), bytes.len()),
        blank_line_before: if tokens.is_empty() { newlines >= 1 } else { newlines >= 2 },
    });
    Ok(tokens)
}

/// The error for a word continued by `.`, spanning the whole dotted run.
fn dotted_word(
    bytes: &[u8],
    start: usize,
    mut i: usize,
    span: impl Fn(usize, usize) -> Span,
) -> Error {
    while bytes
        .get(i)
        .is_some_and(|&b| is_word_continue(b) || b == b'.')
    {
        i += 1;
    }
    Error::new(span(start, i), ErrorKind::DottedWord)
}

/// Lexes the integer starting at `start` (a `-` or a digit), returning it and
/// the offset just past it.
fn integer(
    source: &str,
    start: usize,
    span: impl Fn(usize, usize) -> Span,
) -> Result<(TokenKind, usize), Error> {
    let bytes = source.as_bytes();
    let mut i = start;
    if bytes[i] == b'-' {
        i += 1;
        if !bytes.get(i).is_some_and(u8::is_ascii_digit) {
            return Err(Error::new(span(start, i), ErrorKind::StrayMinus));
        }
    }
    while bytes.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
    }
    let word_like = |b: &u8| is_word_continue(*b) || *b == b'.';
    if bytes.get(i).is_some_and(word_like) {
        while bytes.get(i).is_some_and(word_like) {
            i += 1;
        }
        return Err(Error::new(span(start, i), ErrorKind::InvalidInteger));
    }
    let text = &source[start..i];
    let digits = text.trim_start_matches('-');
    if (digits.len() > 1 && digits.starts_with('0')) || text == "-0" {
        return Err(Error::new(span(start, i), ErrorKind::NonCanonicalInteger));
    }
    let n = text
        .parse()
        .map_err(|_| Error::new(span(start, i), ErrorKind::IntegerOutOfRange))?;
    Ok((TokenKind::Integer(n), i))
}
