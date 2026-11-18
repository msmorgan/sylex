//! Builds the syntax tree from tokens by recursive descent.

use std::mem::discriminant;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::lex::Token;
use crate::lex::TokenKind;
use crate::lex::lex;
use crate::span::FileId;
use crate::span::Span;
use crate::tree::AT_FIELD;
use crate::tree::Arg;
use crate::tree::Call;
use crate::tree::Comment;
use crate::tree::Declaration;
use crate::tree::Entry;
use crate::tree::Field;
use crate::tree::File;
use crate::tree::Header;
use crate::tree::Item;
use crate::tree::METADATA_FIELDS;
use crate::tree::Name;
use crate::tree::Param;
use crate::tree::Record;
use crate::tree::Relation;
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
        in_metadata: false,
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
    /// Whether the parser is inside the value of a metadata field, where
    /// capitalized words are allowed.
    in_metadata: bool,
}

fn is_capitalized(name: Name) -> bool {
    name.ident
        .as_str()
        .starts_with(|c: char| c.is_ascii_uppercase())
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

    /// A word that is not a sort or a constructor, so it must not be
    /// capitalized outside metadata.
    fn lowercase_word(&mut self, expected: &'static str) -> Result<Name> {
        let name = self.word(expected)?;
        self.check_lowercase(name)?;
        Ok(name)
    }

    fn check_lowercase(&self, name: Name) -> Result<()> {
        if is_capitalized(name) && !self.in_metadata {
            return Err(Error::new(
                name.span,
                ErrorKind::CapitalizedWord(name.ident.as_str().to_owned()),
            ));
        }
        Ok(())
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
        let kind = self.lowercase_word("a declaration")?;
        let shape = if self.relation_follows() {
            Shape::Relation(self.relation()?)
        } else if self.named_header_follows() {
            let name = self.lowercase_word("a declaration name")?;
            Shape::Header(self.header(name)?)
        } else {
            Shape::Value(self.value()?)
        };
        Ok(Declaration {
            kind,
            shape,
            span: self.span_from(kind.span),
        })
    }

    /// Whether the tokens after a kind word start a relation between two
    /// sorts: a `(`, which no value starts with.
    fn relation_follows(&self) -> bool {
        self.peek().kind == TokenKind::LParen
    }

    /// `(Subject): Sort`.
    fn relation(&mut self) -> Result<Relation> {
        let open = self.bump().span;
        let subject = self.sort()?;
        self.expect(&TokenKind::RParen, "')'")?;
        self.expect(&TokenKind::Colon, "':'")?;
        let sort = self.sort()?;
        Ok(Relation {
            subject,
            sort,
            span: self.span_from(open),
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

    /// The header after its name.
    fn header(&mut self, name: Name) -> Result<Header> {
        let start = name.span;
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
            let constructor = self.word("a constructor")?;
            if !is_capitalized(constructor) {
                return Err(Error::new(
                    constructor.span,
                    ErrorKind::UncapitalizedConstructor(constructor.ident.as_str().to_owned()),
                ));
            }
            Some(constructor)
        } else {
            None
        };
        let mut at_list = Vec::new();
        let mut at_comments = Vec::new();
        let body = if self.at(&TokenKind::LBrace) {
            let open = self.bump().span;
            Some(self.block(open, &mut at_list, &mut at_comments)?)
        } else {
            None
        };
        Ok(Header {
            name,
            params,
            sort,
            constructor,
            at: at_list,
            at_comments,
            body,
            span: self.span_from(start),
        })
    }

    fn param(&mut self) -> Result<Param> {
        let name = self.lowercase_word("a parameter name")?;
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
                self.check_lowercase(Name { ident, span })?;
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
        let fields = self.sequence(&TokenKind::RBrace, "',' or '}'", Self::field)?;
        Ok(Record {
            fields,
            span: self.span_from(open),
        })
    }

    fn field(&mut self) -> Result<Field> {
        let name = self.lowercase_word("a field name")?;
        self.expect(&TokenKind::Equals, "'='")?;
        self.field_value(name)
    }

    /// A field's value, after its name and `=`.
    fn field_value(&mut self, name: Name) -> Result<Field> {
        let outer = self.in_metadata;
        self.in_metadata |= METADATA_FIELDS.contains(&name.ident.as_str());
        let value = self.value();
        self.in_metadata = outer;
        let value = value?;
        Ok(Field {
            name,
            value,
            span: self.span_from(name.span),
        })
    }

    /// Parses a declaration's block after its `{`: a record whose `at` field,
    /// a list of sorts, goes to `at_list` instead of the record, and the
    /// comments directly before it to `at_comments`.
    fn block(
        &mut self,
        open: Span,
        at_list: &mut Vec<Item<Sort>>,
        at_comments: &mut Vec<Comment>,
    ) -> Result<Record> {
        let mut written: Option<Span> = None;
        let mut sorts = Vec::new();
        let entries = self.sequence(&TokenKind::RBrace, "',' or '}'", |p| {
            let name = p.lowercase_word("a field name")?;
            p.expect(&TokenKind::Equals, "'='")?;
            if name.ident != AT_FIELD {
                return p.field_value(name).map(Some);
            }
            if written.is_some() {
                return Err(Error::new(name.span, ErrorKind::DuplicateAtList));
            }
            written = Some(name.span);
            let bracket = p.expect(&TokenKind::LBracket, "'['")?.span;
            sorts = p.sequence(&TokenKind::RBracket, "',' or ']'", Self::sort)?;
            if sorts.iter().all(|item| item.node().is_none()) {
                return Err(Error::new(p.span_from(bracket), ErrorKind::EmptyAtList));
            }
            Ok(None)
        })?;
        *at_list = sorts;
        // The `at` entry leaves the record with the comments directly before
        // it; a blank line before it is dropped.
        let mut entries = entries;
        if let Some(at) = entries
            .iter()
            .position(|item| matches!(item.entry, Entry::Node(None)))
        {
            let first = entries[..at]
                .iter()
                .rposition(|item| item.comment().is_none())
                .map_or(0, |node| node + 1);
            for item in entries.drain(first..at) {
                let Entry::Comment(comment) = item.entry else {
                    unreachable!("only comments stand between `first` and `at`");
                };
                at_comments.push(comment);
            }
        }
        let fields = entries
            .into_iter()
            .filter_map(|item| {
                let entry = match item.entry {
                    Entry::Comment(comment) => Entry::Comment(comment),
                    Entry::Node(Some(field)) => Entry::Node(field),
                    Entry::Node(None) => return None,
                };
                Some(Item {
                    blank_line_before: item.blank_line_before,
                    entry,
                })
            })
            .collect();
        Ok(Record {
            fields,
            span: self.span_from(open),
        })
    }

    fn call(&mut self) -> Result<Call> {
        let head = self.lowercase_word("a word")?;
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
            let name = self.lowercase_word("an argument name")?;
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
