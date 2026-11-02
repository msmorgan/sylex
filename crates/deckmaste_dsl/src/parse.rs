//! Builds the syntax tree from tokens by recursive descent.

use std::mem::discriminant;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::lex::Token;
use crate::lex::TokenKind;
use crate::lex::lex;
use crate::span::FileId;
use crate::span::Span;
use crate::tree::Arg;
use crate::tree::Call;
use crate::tree::Comment;
use crate::tree::Declaration;
use crate::tree::Entry;
use crate::tree::Field;
use crate::tree::File;
use crate::tree::Item;
use crate::tree::Name;
use crate::tree::NamedShape;
use crate::tree::Param;
use crate::tree::Record;
use crate::tree::Shape;
use crate::tree::Sort;
use crate::tree::Value;
use crate::tree::ValueKind;

/// Parses one `.syl` source file.
///
/// # Errors
///
/// The first lexing or syntax error, located by its span.
pub fn parse(file: FileId, source: &str) -> Result<File> {
    let tokens = lex(file, source)?;
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
        hoisted: Vec::new(),
        prev_end: 0,
    };
    let items = parser.file()?;
    debug_assert_eq!(parser.hoisted, Vec::new());
    Ok(File {
        items,
        span: Span::new(file, 0, source.len()),
    })
}

type Result<T> = std::result::Result<T, Error>;

struct Parser<'t> {
    tokens: &'t [Token],
    pos: usize,
    /// Comments skipped inside an item, waiting to be placed before it.
    hoisted: Vec<(Comment, bool)>,
    /// End of the last significant token consumed.
    prev_end: usize,
}

fn comment_item<T>((comment, blank_line_before): (Comment, bool)) -> Item<T> {
    Item {
        blank_line_before,
        entry: Entry::Comment(comment),
    }
}

impl Parser<'_> {
    /// The `n`th token from the current position, not counting comments.
    fn peek_nth(&self, n: usize) -> &Token {
        let mut seen = 0;
        for token in &self.tokens[self.pos..] {
            if matches!(token.kind, TokenKind::Comment(_)) {
                continue;
            }
            if seen == n || token.kind == TokenKind::Eof {
                return token;
            }
            seen += 1;
        }
        self.tokens.last().expect("the lexer always emits Eof")
    }

    fn peek(&self) -> &Token {
        self.peek_nth(0)
    }

    fn at(&self, kind: &TokenKind) -> bool {
        discriminant(&self.peek().kind) == discriminant(kind)
    }

    /// Consumes the next significant token, hoisting any comments before it.
    fn bump(&mut self) -> Token {
        loop {
            let token = &self.tokens[self.pos];
            if token.kind != TokenKind::Eof {
                self.pos += 1;
            }
            if let TokenKind::Comment(text) = &token.kind {
                self.hoisted.push((
                    Comment {
                        text: text.clone(),
                        span: token.span,
                    },
                    token.blank_line_before,
                ));
                continue;
            }
            self.prev_end = token.span.end;
            return token.clone();
        }
    }

    fn error_here(&self, expected: &'static str) -> Error {
        let token = self.peek();
        Error::new(
            token.span,
            ErrorKind::Expected {
                expected,
                found: token.kind.describe(),
            },
        )
    }

    fn expect(&mut self, kind: &TokenKind, expected: &'static str) -> Result<Token> {
        if self.at(kind) { Ok(self.bump()) } else { Err(self.error_here(expected)) }
    }

    fn word(&mut self, expected: &'static str) -> Result<Name> {
        match self.peek().kind {
            TokenKind::Word(ident) => {
                let span = self.bump().span;
                Ok(Name { ident, span })
            }
            _ => Err(self.error_here(expected)),
        }
    }

    fn span_from(&self, start: Span) -> Span {
        Span::new(start.file, start.start, self.prev_end)
    }

    /// Moves the comments directly at the current position into `items`.
    fn comment_items<T>(&mut self, items: &mut Vec<Item<T>>) {
        while let Some(token) = self.tokens.get(self.pos) {
            let TokenKind::Comment(text) = &token.kind else {
                break;
            };
            items.push(comment_item((
                Comment {
                    text: text.clone(),
                    span: token.span,
                },
                token.blank_line_before,
            )));
            self.pos += 1;
        }
    }

    /// Parses one node with `node`, then appends the comments hoisted from
    /// inside it and the node itself to `items`.
    ///
    /// Hoisted comments are placed where the node started, so the first of
    /// them takes the node's blank-line flag and the node's own flag becomes
    /// false. (A blank line inside the node, before a hoisted comment, is
    /// dropped: it has no place once the comment moves.)
    fn item<T>(
        &mut self,
        items: &mut Vec<Item<T>>,
        node: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<()> {
        let mut blank_line_before = self.peek().blank_line_before;
        let mark = self.hoisted.len();
        let node = node(self)?;
        let mut hoisted = self.hoisted.split_off(mark);
        if let Some(first) = hoisted.first_mut() {
            first.1 = blank_line_before;
            blank_line_before = false;
        }
        items.extend(hoisted.into_iter().map(comment_item));
        items.push(Item {
            blank_line_before,
            entry: Entry::Node(node),
        });
        Ok(())
    }

    /// Parses a comma-separated sequence after its opening delimiter has been
    /// consumed, through the closing delimiter. A trailing comma is allowed.
    fn sequence<T>(
        &mut self,
        close: &TokenKind,
        close_text: &'static str,
        mut node: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<Item<T>>> {
        let mut items = Vec::new();
        loop {
            self.comment_items(&mut items);
            if self.at(close) {
                self.bump();
                return Ok(items);
            }
            self.item(&mut items, &mut node)?;
            self.comment_items(&mut items);
            if self.at(close) {
                self.bump();
                return Ok(items);
            }
            if !self.at(&TokenKind::Comma) {
                return Err(self.error_here(close_text));
            }
            self.bump();
        }
    }

    fn file(&mut self) -> Result<Vec<Item<Declaration>>> {
        let mut items = Vec::new();
        loop {
            self.comment_items(&mut items);
            if self.at(&TokenKind::Eof) {
                return Ok(items);
            }
            self.item(&mut items, Self::declaration)?;
        }
    }

    fn declaration(&mut self) -> Result<Declaration> {
        let kind = self.word("a declaration")?;
        let shape = if self.named_header_follows() {
            Shape::Named(self.named_shape()?)
        } else {
            Shape::Nameless(self.value()?)
        };
        Ok(Declaration {
            kind,
            shape,
            span: self.span_from(kind.span),
        })
    }

    /// Whether the tokens after a kind word start a named header: a word
    /// followed by `(` and then `ident :` or `$ident :` (or `)`), or by `:`,
    /// `=`, `{`, the next declaration's kind word, or the end of the file.
    fn named_header_follows(&self) -> bool {
        if !matches!(self.peek().kind, TokenKind::Word(_)) {
            return false;
        }
        match self.peek_nth(1).kind {
            TokenKind::Colon
            | TokenKind::Equals
            | TokenKind::LBrace
            | TokenKind::Word(_)
            | TokenKind::Eof => true,
            TokenKind::LParen => match self.peek_nth(2).kind {
                TokenKind::Word(_) | TokenKind::Parameter(_) => {
                    self.peek_nth(3).kind == TokenKind::Colon
                }
                TokenKind::RParen => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn named_shape(&mut self) -> Result<NamedShape> {
        let name = self.word("a declaration name")?;
        let params = if self.at(&TokenKind::LParen) {
            let open = self.bump().span;
            let params = self.sequence(&TokenKind::RParen, "',' or ')'", Self::param)?;
            if params.iter().all(|item| item.node().is_none()) {
                return Err(Error::new(
                    self.span_from(open),
                    ErrorKind::EmptyParameterList,
                ));
            }
            params
        } else {
            Vec::new()
        };
        let sort = if self.at(&TokenKind::Colon) {
            self.bump();
            Some(self.sort()?)
        } else {
            None
        };
        let constructor = if self.at(&TokenKind::Equals) {
            self.bump();
            match self.peek().kind {
                TokenKind::Constructor(ident) => Some(Name {
                    ident,
                    span: self.bump().span,
                }),
                _ => return Err(self.error_here("'.constructor'")),
            }
        } else {
            None
        };
        let body = if self.at(&TokenKind::LBrace) {
            let open = self.bump().span;
            Some(self.record(open)?)
        } else {
            None
        };
        Ok(NamedShape {
            name,
            params,
            sort,
            constructor,
            body,
            span: self.span_from(name.span),
        })
    }

    fn param(&mut self) -> Result<Param> {
        let name = self.word("a parameter name")?;
        self.expect(&TokenKind::Colon, "':'")?;
        let sort = self.sort()?;
        let default = if self.at(&TokenKind::Equals) {
            self.bump();
            Some(self.value()?)
        } else {
            None
        };
        Ok(Param {
            name,
            sort,
            default,
            span: self.span_from(name.span),
        })
    }

    fn sort(&mut self) -> Result<Sort> {
        let head = self.word("a sort")?;
        let mut args = Vec::new();
        if self.at(&TokenKind::LParen) {
            let open = self.bump().span;
            loop {
                if self.at(&TokenKind::RParen) {
                    self.bump();
                    break;
                }
                args.push(self.sort()?);
                if self.at(&TokenKind::RParen) {
                    self.bump();
                    break;
                }
                self.expect(&TokenKind::Comma, "',' or ')'")?;
            }
            if args.is_empty() {
                return Err(Error::new(
                    self.span_from(open),
                    ErrorKind::EmptySortArguments,
                ));
            }
        }
        Ok(Sort {
            head,
            args,
            span: self.span_from(head.span),
        })
    }

    fn value(&mut self) -> Result<Value> {
        let start = self.peek().span;
        let kind = match &self.peek().kind {
            TokenKind::String(text) => {
                let text = text.clone();
                self.bump();
                ValueKind::String(text)
            }
            &TokenKind::Integer(n) => {
                self.bump();
                ValueKind::Integer(n)
            }
            TokenKind::LBracket => {
                self.bump();
                ValueKind::List(self.sequence(&TokenKind::RBracket, "',' or ']'", Self::value)?)
            }
            TokenKind::LBrace => {
                let open = self.bump().span;
                ValueKind::Record(self.record(open)?)
            }
            TokenKind::Word(_) => ValueKind::Call(self.call()?),
            &TokenKind::Parameter(ident) => {
                let span = self.bump().span;
                if self.at(&TokenKind::LParen) {
                    return Err(Error::new(self.peek().span, ErrorKind::ParameterArguments));
                }
                ValueKind::Parameter(Name { ident, span })
            }
            _ => return Err(self.error_here("a value")),
        };
        Ok(Value {
            kind,
            span: self.span_from(start),
        })
    }

    /// Parses a record's fields after its `{`.
    fn record(&mut self, open: Span) -> Result<Record> {
        let fields = self.sequence(&TokenKind::RBrace, "',' or '}'", |p| {
            let name = p.word("a field name")?;
            p.expect(&TokenKind::Equals, "'='")?;
            let value = p.value()?;
            Ok(Field {
                name,
                value,
                span: p.span_from(name.span),
            })
        })?;
        Ok(Record {
            fields,
            span: self.span_from(open),
        })
    }

    fn call(&mut self) -> Result<Call> {
        let head = self.word("a word")?;
        let mut args = Vec::new();
        if self.at(&TokenKind::LParen) {
            let open = self.bump().span;
            args = self.sequence(&TokenKind::RParen, "',' or ')'", Self::arg)?;
            if args.iter().all(|item| item.node().is_none()) {
                return Err(Error::new(self.span_from(open), ErrorKind::EmptyCall));
            }
        }
        Ok(Call {
            head,
            args,
            span: self.span_from(head.span),
        })
    }

    fn arg(&mut self) -> Result<Arg> {
        let start = self.peek().span;
        let name = if matches!(self.peek().kind, TokenKind::Word(_))
            && self.peek_nth(1).kind == TokenKind::Equals
        {
            let name = self.word("an argument name")?;
            self.bump();
            Some(name)
        } else {
            None
        };
        let value = self.value()?;
        Ok(Arg {
            name,
            value,
            span: self.span_from(start),
        })
    }
}
