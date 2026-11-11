//! The `.fug` grammar language (Feature Unification Grammar), parsed into a
//! [`Grammar`].
//!
//! A file is a sequence of items. An item begins with an identifier at the
//! file's base indentation, the column of its first token (column 0 in a
//! file, deeper in a grammar written inside indented code); every other line
//! of it is indented further. `//` starts a comment. Every item has the same
//! shape, each part optional and in this order, and each kind of item takes
//! only the parts shown for it below:
//!
//! ```text
//! [keyword] Name [: Parent] [[list]] [-> rhs...] [= value] [{ props }] [where eq, eq, ...]
//! ```
//!
//! ```text
//! root Document
//! feature Number = Singular | Plural          // a leading `|` is allowed
//! category NounPhrase : Parent [Number, Case] { gap, closed_gaps: [Measure], function }
//! word Of = "prep:of"                         // a Closed Word and its lexeme id
//! class Coordinator = ["coord:and", "coord:or"]
//! class CardType { typed }                    // membership from the lexical environment
//! slot NP = ComplementNounPhrase where slot.Case != Nominative
//! slot AmountOrNP = AmountOrNounPhrase { unlike: [NP, Amount] }  // derived where a head selects two
//! high_attachment PrepositionPhrase where $.DeterminativeDistribution = Distributive
//! correlative Both = And
//! idiomatic_coordinate And = IdiomaticCoordinate
//! Name: Lhs -> rhs... { props } where eq, ...  // a Production
//! adjunct PrepositionPhrase { host: VerbPhrase, side: Before, host_kind: Predicate } where eq, ...
//! premodification Participle { head: Verb, host: BareNominal, forms: [...], equations: [Form -> [eq, ...]] }
//! parameters KeywordParameters { head: Keyword, ... }
//! valence VerbPhrase { head: Verb, active: [eq, ...], clusters: { cluster: ..., ... }, ... }
//! coordination NounPhrase { coordinators: And, ... } where eq, ...
//! ```
//!
//! A Production's name is capitalized; the keywords are lowercase. A Schema's
//! name is the Category it is about (the adjunct, the modifier, the phrase,
//! the coordinated Category), and its props are named as the [`SchemaDecl`]
//! fields, as a Production's are named as the [`ProductionDecl`] fields; a
//! `cost` defaults to 1, and to 0 on `valence`. An adjunct's or
//! coordination's equations go in `where`. Category, Production and Schema
//! items are read through the `serde` derives of those types, so their field
//! names, defaults and unknown-field rejection are declared there alone; a
//! flag reads as `true`, and a flag may also be written `key: true`.
//!
//! A right-hand-side symbol is `[role:]Category` (role `head` when omitted),
//! `[role:]"Category"` (set in quotation marks), `'c'` (punctuation),
//! `'c':Binding` (a mark), `'c':Binding:Spoken` (a mark spoken as a word, a
//! sign: `'+':Prefix:Spoken`), or `[role:]Kind(args)` for the other
//! [`SymbolDecl`] kinds (`Class(X)`, `Word(X)`, `Lex(Noun)`, `Free(X)`,
//! `FreeLex(Noun)`, `OtherCardName()`, `Numeral(Cardinal)`, `Numeral(Arabic(false))`,
//! `Hosted(X, Nominal)`, `Attached(X, Suffix)`, `Selected(X, Of, ...)`). An
//! equation is `path = Value` (require), `path = path` (agree),
//! `path := Value` (carry), `path != A | B` (exclude each), or
//! `!(path = A & path = B)` (exclude together), where a path is `$.Feature`
//! or `child.Feature`; `=` is written only in equations. Props are
//! `key: value` or a bare `key` flag; a pair in a prop list is `A -> B`.

use std::fmt::Display;

use dpsi::Ident;
use serde::de;
use serde::de::DeserializeOwned;
use serde::de::DeserializeSeed;
use serde::de::Visitor;

use crate::LoadError;
use crate::frames::SlotKind;
use crate::grammar::CategoryDecl;
use crate::grammar::Equation;
use crate::grammar::Grammar;
use crate::grammar::ProductionDecl;
use crate::grammar::SchemaDecl;

/// Parses `.fug` source into a [`Grammar`].
///
/// # Errors
/// [`LoadError::Format`] naming the line and column of the first defect.
pub fn parse(text: &str) -> Result<Grammar, LoadError> {
    let mut parser = Parser {
        tokens: lex(text)?,
        next: 0,
        item_start: 0,
    };
    let mut builder = Builder::new();
    while parser.next < parser.tokens.len() {
        let item = parser.item()?;
        builder.add(item)?;
    }
    builder.finish()
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    line: usize,
    col: usize,
}

fn fail<T>(at: Pos, message: impl Display) -> Result<T, LoadError> {
    Err(error_at(at, message).load(at))
}

// --- Lexer ------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    Ident(String),
    Str(String),
    Char(char),
    Int(u32),
    Sym(&'static str),
}

struct Token {
    tok: Tok,
    at: Pos,
    /// An identifier at the base indentation: the start of an item.
    item_start: bool,
}

/// Longest first, so `->` is never read as `-`.
const SYMBOLS: [&str; 17] = [
    "->", "!=", ":=", ":", "=", "!", "&", "|", ",", "(", ")", "[", "]", "{", "}", ".", "$",
];

fn lex(text: &str) -> Result<Vec<Token>, LoadError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut base = None;
    for (index, line) in text.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let at = Pos {
                line: index + 1,
                col: i + 1,
            };
            let c = chars[i];
            let rest = &chars[i..];
            let (tok, len) = if c.is_whitespace() {
                i += 1;
                continue;
            } else if starts_with(rest, "//") {
                break;
            } else if c.is_ascii_alphabetic() || c == '_' {
                let len = chars[i..]
                    .iter()
                    .take_while(|c| c.is_ascii_alphanumeric() || **c == '_')
                    .count();
                (Tok::Ident(chars[i..i + len].iter().collect()), len)
            } else if c.is_ascii_digit() {
                let len = chars[i..].iter().take_while(|c| c.is_ascii_digit()).count();
                let digits: String = chars[i..i + len].iter().collect();
                let Ok(n) = digits.parse() else { return fail(at, "integer out of range") };
                (Tok::Int(n), len)
            } else if c == '"' {
                let Some(len) = chars[i + 1..].iter().position(|c| *c == '"') else {
                    return fail(at, "unterminated string");
                };
                (
                    Tok::Str(chars[i + 1..i + 1 + len].iter().collect()),
                    len + 2,
                )
            } else if c == '\'' {
                match chars.get(i + 1..i + 4).or_else(|| chars.get(i + 1..i + 3)) {
                    Some(['\\', 'n', '\'', ..]) => (Tok::Char('\n'), 4),
                    Some([ch, '\'', ..]) if *ch != '\\' => (Tok::Char(*ch), 3),
                    _ => return fail(at, "malformed character literal"),
                }
            } else if let Some(sym) = SYMBOLS.into_iter().find(|s| starts_with(rest, s)) {
                (Tok::Sym(sym), sym.chars().count())
            } else {
                return fail(at, format!("unexpected character `{c}`"));
            };
            let base = *base.get_or_insert(at.col);
            let first_on_line = tokens.last().is_none_or(|t| t.at.line != at.line);
            let item_start = first_on_line && at.col == base && matches!(tok, Tok::Ident(_));
            if first_on_line && at.col < base {
                return fail(at, "a line is indented less than the first item");
            }
            tokens.push(Token {
                tok,
                at,
                item_start,
            });
            i += len;
        }
    }
    Ok(tokens)
}

fn starts_with(chars: &[char], prefix: &str) -> bool {
    let mut chars = chars.iter();
    prefix.chars().all(|c| chars.next() == Some(&c))
}

// --- Syntax -----------------------------------------------------------------

#[derive(Debug, Clone)]
struct Value {
    kind: ValueKind,
    at: Pos,
}

#[derive(Debug, Clone)]
enum ValueKind {
    Ident(String),
    /// `$.Feature` or `child.Feature`.
    Path(String),
    Str(String),
    Char(char),
    Int(u32),
    Call(String, Vec<Value>),
    List(Vec<Value>),
    Props(Props),
    /// `A | B | ...`.
    Alt(Vec<Value>),
    /// `a op b`, `op` one of `=`, `!=`, `:=` and `->`.
    Rel(Box<Value>, &'static str, Box<Value>),
    /// `!(a = A & b = B)`.
    NotAll(Vec<Value>),
    /// A bare prop key: `true`.
    Flag,
    /// A fixed-length sequence the reader builds: a pair's two sides, a
    /// joint exclusion's condition.
    Tuple(Vec<Value>),
}

type Props = Vec<(String, Option<Value>, Pos)>;

struct RhsSymbol {
    role: Option<String>,
    term: Value,
    binding: Option<String>,
    /// Whether the mark is declared spoken (`:Spoken`).
    spoken: bool,
}

struct Item {
    at: Pos,
    keyword: Option<String>,
    name: String,
    parent: Option<String>,
    list: Option<Vec<Value>>,
    rhs: Option<Vec<RhsSymbol>>,
    value: Option<Value>,
    props: Option<Props>,
    equations: Option<Vec<Value>>,
}

struct Parser {
    tokens: Vec<Token>,
    next: usize,
    item_start: usize,
}

impl Parser {
    /// The next token of the current item, if any.
    fn peek(&self) -> Option<&Tok> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, n: usize) -> Option<&Tok> {
        let token = self.tokens.get(self.next + n)?;
        (!token.item_start || self.next + n == self.item_start).then_some(&token.tok)
    }

    fn pos(&self) -> Pos {
        self.tokens
            .get(self.next)
            .or(self.tokens.last())
            .map_or(Pos { line: 1, col: 1 }, |t| t.at)
    }

    fn bump(&mut self) -> Result<(Tok, Pos), LoadError> {
        let at = self.pos();
        let Some(tok) = self.peek().cloned() else {
            return fail(at, "item ends too early");
        };
        self.next += 1;
        Ok((tok, at))
    }

    fn at_sym(&self, sym: &str) -> bool {
        matches!(self.peek(), Some(Tok::Sym(s)) if *s == sym)
    }

    fn eat(&mut self, sym: &str) -> bool {
        let found = self.at_sym(sym);
        self.next += usize::from(found);
        found
    }

    fn expect(&mut self, sym: &str) -> Result<(), LoadError> {
        if self.eat(sym) {
            Ok(())
        } else {
            fail(self.pos(), format!("expected `{sym}`"))
        }
    }

    fn ident(&mut self) -> Result<String, LoadError> {
        match self.bump()? {
            (Tok::Ident(name), _) => Ok(name),
            (_, at) => fail(at, "expected an identifier"),
        }
    }

    fn at_where(&self) -> bool {
        self.peek() == Some(&Tok::Ident("where".into()))
    }

    fn item(&mut self) -> Result<Item, LoadError> {
        self.item_start = self.next;
        let at = self.pos();
        if !self.tokens[self.next].item_start {
            return fail(
                at,
                "an item starts with an identifier at the base indentation",
            );
        }
        let first = self.ident()?;
        let (keyword, name) = if first.starts_with(|c: char| c.is_ascii_lowercase()) {
            if self.at_sym(":") {
                return fail(
                    at,
                    format!("`{first}` is no keyword; a Production's name is capitalized"),
                );
            }
            (Some(first), self.ident()?)
        } else {
            (None, first)
        };
        let parent = if self.eat(":") { Some(self.ident()?) } else { None };
        let list = if self.eat("[") { Some(self.list("]")?) } else { None };
        let rhs = if self.eat("->") { Some(self.rhs()?) } else { None };
        let value = if self.eat("=") { Some(self.value()?) } else { None };
        let props = if self.eat("{") { Some(self.props()?) } else { None };
        let equations = if self.at_where() {
            self.next += 1;
            let mut equations = vec![self.value()?];
            while self.eat(",") {
                equations.push(self.value()?);
            }
            Some(equations)
        } else {
            None
        };
        if self.peek().is_some() {
            return fail(self.pos(), "unexpected token");
        }
        Ok(Item {
            at,
            keyword,
            name,
            parent,
            list,
            rhs,
            value,
            props,
            equations,
        })
    }

    fn rhs(&mut self) -> Result<Vec<RhsSymbol>, LoadError> {
        let mut symbols = Vec::new();
        while self.peek().is_some() && !self.at_sym("{") && !self.at_where() {
            let role = match (self.peek(), self.peek_nth(1)) {
                (Some(Tok::Ident(_)), Some(Tok::Sym(":"))) => {
                    let role = self.ident()?;
                    self.next += 1;
                    Some(role)
                }
                _ => None,
            };
            let term = self.term()?;
            let binding = if matches!(term.kind, ValueKind::Char(_)) && self.eat(":") {
                Some(self.ident()?)
            } else {
                None
            };
            let spoken = if binding.is_some() && self.eat(":") {
                let at = self.pos();
                if self.ident()? != "Spoken" {
                    return fail(at, "a mark's third part is `Spoken`");
                }
                true
            } else {
                false
            };
            symbols.push(RhsSymbol {
                role,
                term,
                binding,
                spoken,
            });
        }
        Ok(symbols)
    }

    fn value(&mut self) -> Result<Value, LoadError> {
        let at = self.pos();
        if self.eat("!") {
            self.expect("(")?;
            let mut all = vec![self.value()?];
            while self.eat("&") {
                all.push(self.value()?);
            }
            self.expect(")")?;
            return Ok(Value {
                kind: ValueKind::NotAll(all),
                at,
            });
        }
        let lhs = self.alternatives()?;
        for op in ["=", "!=", ":=", "->"] {
            if self.eat(op) {
                let rhs = self.alternatives()?;
                return Ok(Value {
                    kind: ValueKind::Rel(Box::new(lhs), op, Box::new(rhs)),
                    at,
                });
            }
        }
        Ok(lhs)
    }

    fn alternatives(&mut self) -> Result<Value, LoadError> {
        let at = self.pos();
        let leading = self.eat("|");
        let first = self.term()?;
        if !leading && !self.at_sym("|") {
            return Ok(first);
        }
        let mut alternatives = vec![first];
        while self.eat("|") {
            alternatives.push(self.term()?);
        }
        Ok(Value {
            kind: ValueKind::Alt(alternatives),
            at,
        })
    }

    fn term(&mut self) -> Result<Value, LoadError> {
        let (tok, at) = self.bump()?;
        let kind = match tok {
            Tok::Sym("$") => {
                self.expect(".")?;
                ValueKind::Path(format!("$.{}", self.ident()?))
            }
            Tok::Ident(name) if self.eat(".") => {
                ValueKind::Path(format!("{name}.{}", self.ident()?))
            }
            Tok::Ident(name) if self.eat("(") => ValueKind::Call(name, self.list(")")?),
            Tok::Ident(name) => ValueKind::Ident(name),
            Tok::Str(text) => ValueKind::Str(text),
            Tok::Char(c) => ValueKind::Char(c),
            Tok::Int(n) => ValueKind::Int(n),
            Tok::Sym("[") => ValueKind::List(self.list("]")?),
            Tok::Sym("{") => ValueKind::Props(self.props()?),
            Tok::Sym(sym) => return fail(at, format!("unexpected `{sym}`")),
        };
        Ok(Value { kind, at })
    }

    /// Comma-separated values up to `close`, a trailing comma allowed.
    fn list(&mut self, close: &str) -> Result<Vec<Value>, LoadError> {
        let mut values = Vec::new();
        while !self.eat(close) {
            values.push(self.value()?);
            if !self.eat(",") {
                self.expect(close)?;
                break;
            }
        }
        Ok(values)
    }

    /// `key: value` and bare `key` entries up to `}`, a trailing comma allowed.
    fn props(&mut self) -> Result<Props, LoadError> {
        let mut props = Vec::new();
        while !self.eat("}") {
            let at = self.pos();
            let key = self.ident()?;
            let value = if self.eat(":") { Some(self.value()?) } else { None };
            props.push((key, value, at));
            if !self.eat(",") {
                self.expect("}")?;
                break;
            }
        }
        Ok(props)
    }
}

// --- Deserializing values into the declaration types --------------------
//
// Every Category, Production and Schema item is read through the `serde`
// derive of its type in `grammar.rs`, which declares the field names,
// defaults and unknown-field policy once. A value takes the shape the target
// type asks for: `A = B` is a pair where a tuple is wanted and an
// `Equation` where an enum is; `Kind(args)` and a bare name are enum
// variants.

/// A deserialization error, located at the innermost value it names.
#[derive(Debug)]
struct Error {
    at: Option<Pos>,
    message: String,
}

impl Error {
    fn at(mut self, at: Pos) -> Self {
        self.at.get_or_insert(at);
        self
    }

    fn load(self, fallback: Pos) -> LoadError {
        let at = self.at.unwrap_or(fallback);
        LoadError::Format(format!("{}:{}: {}", at.line, at.col, self.message))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Self {
            at: None,
            message: message.to_string(),
        }
    }
}

fn error_at(at: Pos, message: impl Display) -> Error {
    Error {
        at: Some(at),
        message: message.to_string(),
    }
}

fn read<T: DeserializeOwned>(value: Value) -> Result<T, LoadError> {
    let at = value.at;
    T::deserialize(value).map_err(|e| e.load(at))
}

impl Value {
    fn new(kind: ValueKind, at: Pos) -> Self {
        Self { kind, at }
    }

    fn is_path(&self) -> bool {
        matches!(self.kind, ValueKind::Path(_))
    }

    /// The path as the name an `Equation` holds.
    fn path(self) -> Result<Self, Error> {
        match self.kind {
            ValueKind::Path(path) => Ok(Self::new(ValueKind::Ident(path), self.at)),
            _ => Err(error_at(self.at, "expected `$.Feature` or `child.Feature`")),
        }
    }

    fn name(self) -> Result<Self, Error> {
        match self.kind {
            ValueKind::Ident(_) => Ok(self),
            _ => Err(error_at(self.at, "expected a name")),
        }
    }

    /// The `Equation` variant a written equation is, and its arguments.
    fn equation(self) -> Result<(String, Vec<Self>), Error> {
        let at = self.at;
        let (variant, args) = match self.kind {
            ValueKind::Rel(a, "=", b) if b.is_path() => ("Agree", vec![a.path()?, b.path()?]),
            ValueKind::Rel(a, "=", b) => ("Require", vec![a.path()?, b.name()?]),
            ValueKind::Rel(a, ":=", b) => ("Carry", vec![a.path()?, b.name()?]),
            ValueKind::Rel(a, "!=", b) => ("Exclude", vec![a.path()?, b.name()?]),
            ValueKind::NotAll(all) => {
                let conditions = all
                    .into_iter()
                    .map(|condition| match condition.kind {
                        ValueKind::Rel(a, "=", b) => Ok(Self::new(
                            ValueKind::Tuple(vec![a.path()?, b.name()?]),
                            condition.at,
                        )),
                        _ => Err(error_at(
                            condition.at,
                            "a joint exclusion is `!(path = A & ...)`",
                        )),
                    })
                    .collect::<Result<_, _>>()?;
                (
                    "ExcludeTogether",
                    vec![Self::new(ValueKind::List(conditions), at)],
                )
            }
            _ => return Err(error_at(at, "expected an equation")),
        };
        Ok((variant.to_owned(), args))
    }

    fn ident(self) -> Result<Ident, LoadError> {
        match self.kind {
            ValueKind::Ident(name) => Ok(Ident::from(name.as_str())),
            _ => fail(self.at, "expected an identifier"),
        }
    }
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        let at = self.at;
        match self.kind {
            ValueKind::Ident(text) => visitor.visit_string(text),
            ValueKind::Int(n) => visitor.visit_u32(n),
            ValueKind::Flag => visitor.visit_bool(true),
            ValueKind::List(values) => visitor.visit_seq(Seq::list(values)?),
            ValueKind::Tuple(values) => visitor.visit_seq(Seq::new(values)),
            ValueKind::Path(_) => Err(error_at(at, "a path is no name; paths go in equations")),
            ValueKind::Char(_) => Err(error_at(at, "a character is no name")),
            ValueKind::Props(props) => {
                let mut map = Map::new(props, at);
                visitor.visit_map(&mut map).map_err(|e| e.at(map.at))
            }
            ValueKind::Str(_) => Err(error_at(at, "a quoted string is no value here")),
            _ => Err(error_at(at, "expected a name, a list or `{ ... }`")),
        }
        .map_err(|e| e.at(at))
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.kind {
            ValueKind::Ident(b) if b == "true" => visitor.visit_bool(true),
            ValueKind::Ident(b) if b == "false" => visitor.visit_bool(false),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.kind {
            ValueKind::Char(c) => visitor.visit_char(c),
            _ => Err(error_at(self.at, "expected a character: ','")),
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_some(self)
    }

    /// `A -> B` is a pair.
    fn deserialize_tuple<V: Visitor<'de>>(self, _: usize, visitor: V) -> Result<V::Value, Error> {
        let at = self.at;
        match self.kind {
            ValueKind::Rel(a, "->", b) => exactly(visitor, vec![*a, *b], at),
            ValueKind::Tuple(values) => exactly(visitor, values, at),
            _ => Err(error_at(at, "expected a pair: `A -> B`")),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        let at = self.at;
        let (variant, args) = match self.kind {
            _ if name == "Equation" => self.equation()?,
            ValueKind::Call(name, args) => (name, args),
            ValueKind::Ident(name) => (name, Vec::new()),
            _ => return Err(error_at(at, "expected a name or `Kind(...)`")),
        };
        visitor
            .visit_enum(Variant {
                name: variant,
                args,
                at,
            })
            .map_err(|e| e.at(at))
    }

    serde::forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 str string bytes byte_buf
        unit unit_struct newtype_struct seq tuple_struct map struct identifier ignored_any
    }
}

/// Visits `values` as a sequence that must be used up.
fn exactly<'de, V: Visitor<'de>>(
    visitor: V,
    values: Vec<Value>,
    at: Pos,
) -> Result<V::Value, Error> {
    let mut seq = Seq::new(values);
    let value = visitor.visit_seq(&mut seq)?;
    match seq.values.next() {
        None => Ok(value),
        Some(extra) => Err(error_at(extra.at, "one argument too many").at(at)),
    }
}

/// Elements to visit. `a != A | B` stands for one exclusion per alternative.
struct Seq {
    values: std::vec::IntoIter<Value>,
}

impl Seq {
    /// A written list, which names nothing twice.
    fn list(values: Vec<Value>) -> Result<Self, Error> {
        for (i, value) in values.iter().enumerate() {
            if let ValueKind::Ident(name) = &value.kind
                && values[..i]
                    .iter()
                    .any(|v| matches!(&v.kind, ValueKind::Ident(n) if n == name))
            {
                return Err(error_at(value.at, format!("`{name}` is listed twice")));
            }
        }
        Ok(Self::new(values))
    }

    fn new(values: Vec<Value>) -> Self {
        let mut out: Vec<Value> = Vec::new();
        for value in values {
            match value.kind {
                ValueKind::Rel(a, "!=", b) => {
                    let alternatives = match b.kind {
                        ValueKind::Alt(alternatives) => alternatives,
                        kind => vec![Value::new(kind, b.at)],
                    };
                    for alternative in alternatives {
                        out.push(Value::new(
                            ValueKind::Rel(a.clone(), "!=", Box::new(alternative)),
                            value.at,
                        ));
                    }
                }
                kind => out.push(Value::new(kind, value.at)),
            }
        }
        Self {
            values: out.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for Seq {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Error> {
        self.values
            .next()
            .map(|value| seed.deserialize(value))
            .transpose()
    }
}

/// `{ key: value, flag }`, located at the key last read.
struct Map {
    entries: std::vec::IntoIter<(String, Option<Value>, Pos)>,
    value: Option<(String, Value)>,
    at: Pos,
    start: Pos,
}

impl Map {
    fn new(props: Props, at: Pos) -> Self {
        Self {
            entries: props.into_iter(),
            value: None,
            at,
            start: at,
        }
    }
}

impl<'de> de::MapAccess<'de> for Map {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Error> {
        let Some((key, value, at)) = self.entries.next() else {
            // A missing field is the whole block's.
            self.at = self.start;
            return Ok(None);
        };
        self.at = at;
        self.value = Some((
            key.clone(),
            value.unwrap_or(Value::new(ValueKind::Flag, at)),
        ));
        seed.deserialize(Value::new(ValueKind::Ident(key), at))
            .map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Error> {
        let (key, value) = self
            .value
            .take()
            .expect("serde reads a key before its value");
        let flag = matches!(value.kind, ValueKind::Flag);
        let at = value.at;
        seed.deserialize(value)
            .map_err(|e| if flag { error_at(at, format!("`{key}` takes a value")) } else { e })
    }
}

/// An enum variant and its arguments: `Kind(args)`, a bare `Kind`, or the
/// variant an equation names.
struct Variant {
    name: String,
    args: Vec<Value>,
    at: Pos,
}

impl<'de> de::EnumAccess<'de> for Variant {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self), Error> {
        let name = Value::new(ValueKind::Ident(self.name.clone()), self.at);
        Ok((seed.deserialize(name)?, self))
    }
}

impl<'de> de::VariantAccess<'de> for Variant {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.args.first() {
            None => Ok(()),
            Some(arg) => Err(error_at(
                arg.at,
                format!("`{}` takes no arguments", self.name),
            )),
        }
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Error> {
        match <[Value; 1]>::try_from(self.args) {
            Ok([arg]) => seed.deserialize(arg),
            Err(_) => Err(error_at(
                self.at,
                format!("`{}` takes one argument", self.name),
            )),
        }
    }

    fn tuple_variant<V: Visitor<'de>>(self, _: usize, visitor: V) -> Result<V::Value, Error> {
        exactly(visitor, self.args, self.at)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        match <[Value; 1]>::try_from(self.args) {
            Ok([props]) => de::Deserializer::deserialize_any(props, visitor),
            Err(_) => Err(error_at(
                self.at,
                format!("`{}` takes `{{ ... }}`", self.name),
            )),
        }
    }
}

// --- Items ------------------------------------------------------------------

/// The Schema kinds by keyword (the `SchemaDecl` variant, snake-cased), with
/// the field the item's name fills.
const SCHEMAS: [(&str, &str); 5] = [
    ("adjunct", "adjunct"),
    ("premodification", "modifier"),
    ("parameters", "phrase"),
    ("valence", "phrase"),
    ("coordination", "category"),
];

impl Item {
    fn ident(&self, name: &str) -> Value {
        Value::new(ValueKind::Ident(name.to_owned()), self.at)
    }

    /// The item's props with its positional parts as the `fields` they
    /// fill, which no written prop may also set; `where_field`, if any, is
    /// the field its `where` equations fill.
    fn record(
        &mut self,
        fields: Vec<(&str, Option<Value>)>,
        where_field: Option<&str>,
    ) -> Result<Value, LoadError> {
        let written = self.props.take().unwrap_or_default();
        let reserved =
            |key: &str| fields.iter().any(|(k, _)| *k == key) || where_field == Some(key);
        if let Some((key, _, at)) = written.iter().find(|(key, ..)| reserved(key)) {
            return fail(
                *at,
                format!("`{key}` is written in the item's own syntax, not as a setting"),
            );
        }
        let mut props: Props = Vec::new();
        for (key, value) in fields {
            if let Some(value) = value {
                props.push((key.to_owned(), Some(value), self.at));
            }
        }
        if let Some(field) = where_field
            && let Some(equations) = self.equations.take()
        {
            props.push((
                field.to_owned(),
                Some(Value::new(ValueKind::List(equations), self.at)),
                self.at,
            ));
        }
        props.extend(written);
        Ok(Value::new(ValueKind::Props(props), self.at))
    }

    fn value(&mut self) -> Result<Value, LoadError> {
        self.value
            .take()
            .map_or_else(|| fail(self.at, "expected `= ...`"), Ok)
    }

    fn equations(&mut self) -> Result<Vec<Equation>, LoadError> {
        let at = self.at;
        self.equations.take().map_or_else(
            || Ok(Vec::new()),
            |eqs| read(Value::new(ValueKind::List(eqs), at)),
        )
    }

    /// Rejects any part the item's kind did not take.
    fn finish(self) -> Result<(), LoadError> {
        let parts = [
            ("`: Parent`", self.parent.is_some()),
            ("`[...]`", self.list.is_some()),
            ("`->`", self.rhs.is_some()),
            ("`=`", self.value.is_some()),
            ("`{ ... }`", self.props.is_some()),
            ("`where`", self.equations.is_some()),
        ];
        match parts.into_iter().find(|(_, present)| *present) {
            None => Ok(()),
            Some((part, _)) => fail(self.at, format!("this item takes no {part}")),
        }
    }
}

/// A right-hand-side symbol as the `SymbolDecl` variant it writes:
/// `role:B` is `Cat(role, B)`, `"B"` is `Quoted(head, B)`, `','` is
/// `Punct(',')`, `'—':Free` is `Mark('—', Free)`, `'+':Prefix:Spoken` is
/// `SpokenMark('+', Prefix)`, and `Kind(args)` is
/// `Kind(role, args)`, a `Selected`'s Closed Words gathered in a list.
fn symbol(symbol: RhsSymbol) -> Result<Value, LoadError> {
    let RhsSymbol {
        role,
        term,
        binding,
        spoken,
    } = symbol;
    let at = term.at;
    let char_term = matches!(term.kind, ValueKind::Char(_));
    if char_term && role.is_some() {
        return fail(at, "punctuation names no child");
    }
    let role = Value::new(
        ValueKind::Ident(role.unwrap_or_else(|| "head".to_owned())),
        at,
    );
    let (kind, args) = match term.kind {
        ValueKind::Char(_) => match binding {
            None => ("Punct".to_owned(), vec![term]),
            Some(binding) => (
                if spoken { "SpokenMark" } else { "Mark" }.to_owned(),
                vec![term, Value::new(ValueKind::Ident(binding), at)],
            ),
        },
        ValueKind::Ident(_) => ("Cat".to_owned(), vec![role, term]),
        ValueKind::Str(category) => (
            "Quoted".to_owned(),
            vec![role, Value::new(ValueKind::Ident(category), at)],
        ),
        ValueKind::Call(kind, mut args) => {
            if kind == "Selected" && !args.is_empty() {
                let words = args.split_off(1);
                args.push(Value::new(ValueKind::List(words), at));
            }
            args.insert(0, role);
            (kind, args)
        }
        _ => return fail(at, "expected a right-hand-side symbol"),
    };
    Ok(Value::new(ValueKind::Call(kind, args), at))
}

fn production(item: &mut Item) -> Result<ProductionDecl, LoadError> {
    let (Some(lhs), Some(rhs)) = (item.parent.take(), item.rhs.take()) else {
        return fail(item.at, "a Production is `Name: Lhs -> ...`");
    };
    let rhs = rhs.into_iter().map(symbol).collect::<Result<_, _>>()?;
    let name = item.ident(&item.name.clone());
    let lhs = item.ident(&lhs);
    let rhs = Value::new(ValueKind::List(rhs), item.at);
    let record = item.record(
        vec![("name", Some(name)), ("lhs", Some(lhs)), ("rhs", Some(rhs))],
        Some("equations"),
    )?;
    read(record)
}

fn category(item: &mut Item) -> Result<CategoryDecl, LoadError> {
    let parent = item.parent.take().map(|parent| item.ident(&parent));
    let at = item.at;
    let features = item
        .list
        .take()
        .map(|features| Value::new(ValueKind::List(features), at));
    read(item.record(vec![("category", parent), ("features", features)], None)?)
}

fn schema(keyword: &str, item: &mut Item) -> Result<SchemaDecl, LoadError> {
    let Some((_, field)) = SCHEMAS.iter().find(|(k, _)| *k == keyword) else {
        return fail(
            item.at,
            format!("unknown keyword `{keyword}`; a Production's name is capitalized"),
        );
    };
    let name = item.ident(&item.name.clone());
    // Only an adjunct's and a coordination's equations are written in `where`.
    let where_field = matches!(keyword, "adjunct" | "coordination").then_some("equations");
    let record = item.record(vec![(field, Some(name))], where_field)?;
    read(Value::new(
        ValueKind::Call(keyword.to_owned(), vec![record]),
        item.at,
    ))
}

/// The Grammar's sections, filled item by item.
struct Builder {
    root: Option<Ident>,
    grammar: Grammar,
}

impl Builder {
    fn new() -> Self {
        Self {
            root: None,
            grammar: Grammar::default(),
        }
    }

    fn finish(self) -> Result<Grammar, LoadError> {
        let Some(root) = self.root else {
            return Err(LoadError::Format("no `root` item".into()));
        };
        Ok(Grammar {
            root,
            ..self.grammar
        })
    }

    fn add(&mut self, mut item: Item) -> Result<(), LoadError> {
        let at = item.at;
        let name = Ident::from(item.name.as_str());
        let grammar = &mut self.grammar;
        let duplicate = match item.keyword.take().as_deref() {
            None => {
                grammar.productions.push(production(&mut item)?);
                false
            }
            Some("root") if self.root.is_some() => return fail(at, "`root` is given twice"),
            Some("root") => {
                self.root = Some(name);
                false
            }
            Some("feature") => {
                let value = item.value()?;
                let values = match value.kind {
                    ValueKind::Alt(values) => values,
                    kind => vec![Value::new(kind, value.at)],
                };
                let values = read(Value::new(ValueKind::List(values), value.at))?;
                grammar.features.insert(name, values).is_some()
            }
            Some("category") => {
                let decl = category(&mut item)?;
                grammar.categories.insert(name, decl).is_some()
            }
            Some("word") => {
                let lexeme = lexeme(item.value()?)?;
                grammar.closed_words.insert(name, lexeme).is_some()
            }
            Some("class") if item.value.is_some() => {
                let value = item.value()?;
                let ValueKind::List(members) = value.kind else {
                    return fail(value.at, "a class is `class Name = [\"lexeme\", ...]`");
                };
                let members = members.into_iter().map(lexeme).collect::<Result<_, _>>()?;
                grammar.lexical_classes.insert(name, members).is_some()
            }
            Some("class") => {
                let props = item.props.take().unwrap_or_default();
                let typed = |value: &Option<Value>| {
                    value
                        .as_ref()
                        .is_none_or(|v| matches!(&v.kind, ValueKind::Ident(b) if b == "true"))
                };
                if !matches!(props.as_slice(), [(key, value, _)] if key == "typed" && typed(value))
                {
                    return fail(
                        at,
                        "a class is `class Name = [...]` or `class Name { typed }`",
                    );
                }
                grammar.typed_classes.push(name);
                false
            }
            Some("slot") if item.props.is_some() => {
                let shape = "an unlike slot is `slot Name = Category { unlike: [Kind, ...] }`";
                let mut props = item.props.take().into_iter().flatten().collect::<Vec<_>>();
                let (Some((key, Some(kinds), _)), true) = (props.pop(), props.is_empty()) else {
                    return fail(at, shape);
                };
                if key != "unlike" {
                    return fail(at, shape);
                }
                let kinds: Vec<SlotKind> = read(kinds)?;
                if kinds.len() < 2 {
                    return fail(at, "an unlike slot names two or more kinds");
                }
                let category = item.value()?.ident()?;
                let duplicate = grammar.unlike_slots.iter().any(|(n, _)| *n == name);
                let index = u8::try_from(grammar.unlike_slots.len())
                    .map_err(|_| LoadError::Format("more than 256 unlike slots".into()))?;
                grammar.unlike_slots.push((name, kinds));
                grammar
                    .slot_categories
                    .insert(SlotKind::Unlike(index), category);
                duplicate
            }
            Some("slot") => {
                let slot: SlotKind = read(item.ident(&item.name.clone()))?;
                let category = item.value()?.ident()?;
                let equations = item.equations()?;
                if !equations.is_empty() {
                    grammar.slot_equations.insert(slot, equations);
                }
                grammar.slot_categories.insert(slot, category).is_some()
            }
            Some("high_attachment") => {
                let equations = item.equations()?;
                let [Equation::Require(path, value)] = equations.as_slice() else {
                    return fail(at, "a high attachment is `where $.Feature = Value`");
                };
                let Some(feature) = path.strip_prefix("$.") else {
                    return fail(at, "a high attachment is `where $.Feature = Value`");
                };
                grammar
                    .high_attachment
                    .push((name, Ident::from(feature), *value));
                false
            }
            Some("correlative") => {
                grammar.correlatives.push((name, item.value()?.ident()?));
                false
            }
            Some("idiomatic_coordinate") => {
                grammar
                    .idiomatic_coordinates
                    .push((name, item.value()?.ident()?));
                false
            }
            Some(keyword) => {
                grammar.schemas.push(schema(keyword, &mut item)?);
                false
            }
        };
        if duplicate {
            return fail(at, format!("`{name}` is declared twice"));
        }
        item.finish()
    }
}

/// A quoted lexeme id.
fn lexeme(value: Value) -> Result<Ident, LoadError> {
    match value.kind {
        ValueKind::Str(lexeme) => Ok(Ident::from(lexeme.as_str())),
        _ => fail(value.at, "a lexeme id is quoted: \"prep:of\""),
    }
}
