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
//! coordination's equations go in `where`.
//!
//! A right-hand-side symbol is `[role:]Category` (role `head` when omitted),
//! `[role:]"Category"` (set in quotation marks), `'c'` (punctuation),
//! `'c':Binding` (a mark), or `[role:]Kind(args)` for the other
//! [`SymbolDecl`] kinds (`Class(X)`, `Word(X)`, `Lex(Noun)`, `Free(X)`,
//! `FreeLex(Noun)`, `Numeral(Cardinal)`, `Numeral(Arabic(false))`,
//! `Hosted(X, Nominal)`, `Attached(X, Suffix)`, `Selected(X, Of, ...)`). An
//! equation is `path = Value` (require), `path = path` (agree),
//! `path := Value` (carry), `path != A | B` (exclude each), or
//! `!(path = A & path = B)` (exclude together), where a path is `$.Feature`
//! or `child.Feature`. Props are `key: value` or a bare `key` flag; a pair in
//! a prop list is `A = B` (features) or `A -> B` (a map).

use std::collections::BTreeMap;
use std::fmt::Display;

use dpsi::Ident;
use serde::de::DeserializeOwned;
use serde::de::value::Error as ValueError;
use serde::de::value::StrDeserializer;

use crate::LoadError;
use crate::frames::SlotKind;
use crate::grammar::AdjunctClustersDecl;
use crate::grammar::CategoryDecl;
use crate::grammar::ClustersDecl;
use crate::grammar::Equation;
use crate::grammar::Grammar;
use crate::grammar::ProductionDecl;
use crate::grammar::SchemaDecl;
use crate::grammar::SymbolDecl;
use crate::grammar::default_cost;

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
    Err(LoadError::Format(format!(
        "{}:{}: {message}",
        at.line, at.col
    )))
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
}

type Props = Vec<(String, Option<Value>, Pos)>;

struct RhsSymbol {
    role: Option<String>,
    term: Value,
    binding: Option<String>,
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
            symbols.push(RhsSymbol {
                role,
                term,
                binding,
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

// --- Values -----------------------------------------------------------------

impl Value {
    fn ident(self) -> Result<Ident, LoadError> {
        match self.kind {
            ValueKind::Ident(name) => Ok(Ident::from(name.as_str())),
            _ => fail(self.at, "expected an identifier"),
        }
    }

    fn idents(self) -> Result<Vec<Ident>, LoadError> {
        distinct(self.list()?)
    }

    fn list(self) -> Result<Vec<Value>, LoadError> {
        match self.kind {
            ValueKind::List(values) => Ok(values),
            _ => fail(self.at, "expected a list"),
        }
    }

    fn variant<T: DeserializeOwned>(self) -> Result<T, LoadError> {
        let at = self.at;
        variant(&self.ident()?, at)
    }

    fn bool(self) -> Result<bool, LoadError> {
        match self.kind {
            ValueKind::Ident(b) if b == "true" => Ok(true),
            ValueKind::Ident(b) if b == "false" => Ok(false),
            _ => fail(self.at, "expected `true` or `false`"),
        }
    }

    /// `A op B`, with `op` the expected one.
    fn pair(self, expected: &str) -> Result<(Value, Value), LoadError> {
        match self.kind {
            ValueKind::Rel(a, op, b) if op == expected => Ok((*a, *b)),
            _ => fail(self.at, format!("expected `A {expected} B`")),
        }
    }

    /// A list of `A op B`, each side read by its function.
    fn pairs<A, B>(
        self,
        op: &str,
        a: fn(Value) -> Result<A, LoadError>,
        b: fn(Value) -> Result<B, LoadError>,
    ) -> Result<Vec<(A, B)>, LoadError> {
        self.list()?
            .into_iter()
            .map(|v| v.pair(op).and_then(|(x, y)| Ok((a(x)?, b(y)?))))
            .collect()
    }

    fn path(self) -> Result<Ident, LoadError> {
        match self.kind {
            ValueKind::Path(path) => Ok(Ident::from(path.as_str())),
            _ => fail(self.at, "expected `$.Feature` or `child.Feature`"),
        }
    }

    /// Pushes the equations one written equation stands for.
    fn lower_into(self, out: &mut Vec<Equation>) -> Result<(), LoadError> {
        let at = self.at;
        match self.kind {
            ValueKind::NotAll(all) => {
                let pairs = all
                    .into_iter()
                    .map(|v| v.pair("=").and_then(|(a, b)| Ok((a.path()?, b.ident()?))))
                    .collect::<Result<_, _>>()?;
                out.push(Equation::ExcludeTogether(pairs));
            }
            ValueKind::Rel(a, "!=", b) => {
                let path = a.path()?;
                let values = match b.kind {
                    ValueKind::Alt(values) => values,
                    _ => vec![*b],
                };
                for value in values {
                    out.push(Equation::Exclude(path, value.ident()?));
                }
            }
            ValueKind::Rel(a, op, b) => {
                let path = a.path()?;
                out.push(match (op, b.kind) {
                    ("=", ValueKind::Path(other)) => {
                        Equation::Agree(path, Ident::from(other.as_str()))
                    }
                    ("=", ValueKind::Ident(value)) => {
                        Equation::Require(path, Ident::from(value.as_str()))
                    }
                    (":=", ValueKind::Ident(value)) => {
                        Equation::Carry(path, Ident::from(value.as_str()))
                    }
                    _ => return fail(at, "malformed equation"),
                });
            }
            _ => return fail(at, "expected an equation"),
        }
        Ok(())
    }

    fn equations(self) -> Result<Vec<Equation>, LoadError> {
        lower(self.list()?)
    }

    fn props(self) -> Result<Fields, LoadError> {
        let at = self.at;
        match self.kind {
            ValueKind::Props(props) => Fields::new(props, at),
            _ => fail(at, "expected `{ ... }`"),
        }
    }
}

/// The equations a written list of equations stands for.
fn lower(values: Vec<Value>) -> Result<Vec<Equation>, LoadError> {
    let mut out = Vec::new();
    for value in values {
        value.lower_into(&mut out)?;
    }
    Ok(out)
}

/// Identifiers, none named twice: a feature's values, a Category's features,
/// a setting's children.
fn distinct(values: Vec<Value>) -> Result<Vec<Ident>, LoadError> {
    let mut idents = Vec::new();
    for value in values {
        let at = value.at;
        let ident = value.ident()?;
        if idents.contains(&ident) {
            return fail(at, format!("`{ident}` is listed twice"));
        }
        idents.push(ident);
    }
    Ok(idents)
}

/// A built-in enum's unit variant, by its Rust name.
fn variant<T: DeserializeOwned>(name: &str, at: Pos) -> Result<T, LoadError> {
    T::deserialize(StrDeserializer::<ValueError>::new(name)).or_else(|e| fail(at, e))
}

/// Props by key, each taken once; [`Fields::finish`] rejects the rest.
struct Fields {
    at: Pos,
    map: BTreeMap<String, (Option<Value>, Pos)>,
}

impl Fields {
    fn new(props: Props, at: Pos) -> Result<Self, LoadError> {
        let mut map = BTreeMap::new();
        for (key, value, key_at) in props {
            if map.insert(key.clone(), (value, key_at)).is_some() {
                return fail(key_at, format!("`{key}` is given twice"));
            }
        }
        Ok(Self { at, map })
    }

    fn value(&mut self, key: &str) -> Result<Option<Value>, LoadError> {
        match self.map.remove(key) {
            None => Ok(None),
            Some((Some(value), _)) => Ok(Some(value)),
            Some((None, at)) => fail(at, format!("`{key}` takes a value")),
        }
    }

    fn required(&mut self, key: &str) -> Result<Value, LoadError> {
        let at = self.at;
        self.value(key)?
            .map_or_else(|| fail(at, format!("`{key}` is required")), Ok)
    }

    fn flag(&mut self, key: &str) -> Result<bool, LoadError> {
        match self.map.remove(key) {
            None => Ok(false),
            Some((None, _)) => Ok(true),
            Some((Some(value), _)) => {
                fail(value.at, format!("`{key}` is a flag and takes no value"))
            }
        }
    }

    fn ident(&mut self, key: &str) -> Result<Option<Ident>, LoadError> {
        self.value(key)?.map(Value::ident).transpose()
    }

    fn idents(&mut self, key: &str) -> Result<Vec<Ident>, LoadError> {
        Ok(self
            .value(key)?
            .map(Value::idents)
            .transpose()?
            .unwrap_or_default())
    }

    fn equations(&mut self, key: &str) -> Result<Vec<Equation>, LoadError> {
        Ok(self
            .value(key)?
            .map(Value::equations)
            .transpose()?
            .unwrap_or_default())
    }

    fn pairs<A, B>(
        &mut self,
        key: &str,
        op: &str,
        a: fn(Value) -> Result<A, LoadError>,
        b: fn(Value) -> Result<B, LoadError>,
    ) -> Result<Vec<(A, B)>, LoadError> {
        Ok(self
            .value(key)?
            .map(|v| v.pairs(op, a, b))
            .transpose()?
            .unwrap_or_default())
    }

    fn cost(&mut self, default: u32) -> Result<u32, LoadError> {
        match self.value("cost")? {
            None => Ok(default),
            Some(Value {
                kind: ValueKind::Int(n),
                ..
            }) => Ok(n),
            Some(value) => fail(value.at, "expected an integer cost"),
        }
    }

    fn char(&mut self, key: &str) -> Result<Option<char>, LoadError> {
        match self.value(key)? {
            None => Ok(None),
            Some(Value {
                kind: ValueKind::Char(c),
                ..
            }) => Ok(Some(c)),
            Some(value) => fail(value.at, "expected a character"),
        }
    }

    fn finish(self) -> Result<(), LoadError> {
        match self.map.into_iter().next() {
            None => Ok(()),
            Some((key, (_, at))) => fail(at, format!("unknown setting `{key}`")),
        }
    }
}

// --- Items ------------------------------------------------------------------

impl Item {
    fn fields(&mut self) -> Result<Fields, LoadError> {
        Fields::new(self.props.take().unwrap_or_default(), self.at)
    }

    fn equations(&mut self) -> Result<Vec<Equation>, LoadError> {
        self.equations
            .take()
            .map(lower)
            .transpose()
            .map(Option::unwrap_or_default)
    }

    fn value(&mut self) -> Result<Value, LoadError> {
        self.value
            .take()
            .map_or_else(|| fail(self.at, "expected `= ...`"), Ok)
    }

    fn name(&self) -> Ident {
        Ident::from(self.name.as_str())
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

struct Builder {
    root: Option<Ident>,
    grammar: Grammar,
}

impl Builder {
    fn new() -> Self {
        let grammar = Grammar {
            root: Ident::from(""),
            features: BTreeMap::new(),
            categories: BTreeMap::new(),
            closed_words: BTreeMap::new(),
            lexical_classes: BTreeMap::new(),
            typed_classes: Vec::new(),
            slot_categories: BTreeMap::new(),
            slot_equations: BTreeMap::new(),
            high_attachment: Vec::new(),
            productions: Vec::new(),
            schemas: Vec::new(),
            correlatives: Vec::new(),
            idiomatic_coordinates: Vec::new(),
        };
        Self {
            root: None,
            grammar,
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
        let name = item.name();
        let duplicate = match item.keyword.take().as_deref() {
            None => {
                let production = production(&mut item)?;
                self.grammar.productions.push(production);
                false
            }
            Some("root") if self.root.is_some() => return fail(at, "`root` is given twice"),
            Some("root") => {
                self.root = Some(name);
                false
            }
            Some("feature") => {
                let values = match item.value()? {
                    Value {
                        kind: ValueKind::Alt(values),
                        ..
                    } => values,
                    value => vec![value],
                };
                let values = distinct(values)?;
                self.grammar.features.insert(name, values).is_some()
            }
            Some("category") => {
                let mut fields = item.fields()?;
                let decl = CategoryDecl {
                    category: item.parent.take().map(|p| Ident::from(p.as_str())),
                    features: distinct(item.list.take().unwrap_or_default())?,
                    gap: fields.flag("gap")?,
                    closed_gaps: fields.idents("closed_gaps")?,
                    function: fields.flag("function")?,
                };
                fields.finish()?;
                self.grammar.categories.insert(name, decl).is_some()
            }
            Some("word") => {
                let lexeme = match item.value()?.kind {
                    ValueKind::Str(lexeme) => Ident::from(lexeme.as_str()),
                    _ => return fail(at, "a word is `word Name = \"lexeme\"`"),
                };
                self.grammar.closed_words.insert(name, lexeme).is_some()
            }
            Some("class") => self.class(&mut item, name)?,
            Some("slot") => {
                let slot: SlotKind = variant(&item.name, at)?;
                let category = item.value()?.ident()?;
                let equations = item.equations()?;
                if !equations.is_empty() {
                    self.grammar.slot_equations.insert(slot, equations);
                }
                self.grammar
                    .slot_categories
                    .insert(slot, category)
                    .is_some()
            }
            Some("high_attachment") => {
                let equations = item.equations()?;
                let [Equation::Require(path, value)] = equations.as_slice() else {
                    return fail(at, "a high attachment is `where $.Feature = Value`");
                };
                let Some(feature) = path.strip_prefix("$.") else {
                    return fail(at, "a high attachment is `where $.Feature = Value`");
                };
                let entry = (name, Ident::from(feature), *value);
                self.grammar.high_attachment.push(entry);
                false
            }
            Some("correlative") => {
                let pair = (name, item.value()?.ident()?);
                self.grammar.correlatives.push(pair);
                false
            }
            Some("idiomatic_coordinate") => {
                let pair = (name, item.value()?.ident()?);
                self.grammar.idiomatic_coordinates.push(pair);
                false
            }
            Some(kind) => {
                let schema = schema(kind, &mut item)?;
                self.grammar.schemas.push(schema);
                false
            }
        };
        if duplicate {
            return fail(at, format!("`{name}` is declared twice"));
        }
        item.finish()
    }

    /// `class Name = ["lexeme", ...]`, or `class Name { typed }` for a class
    /// the lexical environment declares.
    fn class(&mut self, item: &mut Item, name: Ident) -> Result<bool, LoadError> {
        let mut fields = item.fields()?;
        let typed = fields.flag("typed")?;
        fields.finish()?;
        if typed {
            self.grammar.typed_classes.push(name);
            return Ok(false);
        }
        let members = item
            .value()?
            .list()?
            .into_iter()
            .map(|member| match member.kind {
                ValueKind::Str(lexeme) => Ok(Ident::from(lexeme.as_str())),
                _ => fail(member.at, "a class member is a quoted lexeme id"),
            })
            .collect::<Result<_, _>>()?;
        Ok(self.grammar.lexical_classes.insert(name, members).is_some())
    }
}

fn production(item: &mut Item) -> Result<ProductionDecl, LoadError> {
    let Some(lhs) = item.parent.take() else {
        return fail(item.at, "a Production is `Name: Lhs -> ...`");
    };
    let Some(rhs) = item.rhs.take() else {
        return fail(item.at, "a Production is `Name: Lhs -> ...`");
    };
    let mut fields = item.fields()?;
    let unadjoined = fields.pairs("unadjoined", "->", Value::ident, Value::idents)?;
    let elliptical = fields
        .value("elliptical")?
        .map(|v| v.list()?.into_iter().map(Value::variant).collect())
        .transpose()?;
    let production = ProductionDecl {
        name: item.name(),
        lhs: Ident::from(lhs.as_str()),
        rhs: rhs.into_iter().map(symbol).collect::<Result<_, _>>()?,
        equations: item.equations()?,
        cost: fields.cost(default_cost())?,
        heavy: fields.idents("heavy")?,
        light: fields.idents("light")?,
        unextended: fields.idents("unextended")?,
        unadjoined,
        transparent: fields.idents("transparent")?,
        elliptical,
        unclosed: fields.flag("unclosed")?,
        scoped: fields.idents("scoped")?,
        uncoordinated: fields.idents("uncoordinated")?,
        unlinked: fields.idents("unlinked")?,
        coordinated: fields.idents("coordinated")?,
        unshared: fields.idents("unshared")?,
    };
    fields.finish()?;
    Ok(production)
}

fn symbol(symbol: RhsSymbol) -> Result<SymbolDecl, LoadError> {
    let RhsSymbol {
        role,
        term,
        binding,
    } = symbol;
    let at = term.at;
    let has_role = role.is_some();
    let role = Ident::from(role.as_deref().unwrap_or("head"));
    let decl = match term.kind {
        ValueKind::Char(c) if !has_role => match binding {
            None => SymbolDecl::Punct(c),
            Some(binding) => SymbolDecl::Mark(c, variant(&binding, at)?),
        },
        ValueKind::Ident(category) => SymbolDecl::Cat(role, Ident::from(category.as_str())),
        ValueKind::Str(category) => SymbolDecl::Quoted(role, Ident::from(category.as_str())),
        ValueKind::Call(kind, args) => call_symbol(role, &kind, args, at)?,
        _ => return fail(at, "expected a right-hand-side symbol"),
    };
    Ok(decl)
}

fn call_symbol(
    role: Ident,
    kind: &str,
    args: Vec<Value>,
    at: Pos,
) -> Result<SymbolDecl, LoadError> {
    let mut args = args.into_iter();
    let mut arg = || {
        args.next()
            .map_or_else(|| fail(at, format!("`{kind}` lacks an argument")), Ok)
    };
    let decl = match kind {
        "Class" => SymbolDecl::Class(role, arg()?.ident()?),
        "Word" => SymbolDecl::Word(role, arg()?.ident()?),
        "Free" => SymbolDecl::Free(role, arg()?.ident()?),
        "Lex" => SymbolDecl::Lex(role, arg()?.variant()?),
        "FreeLex" => SymbolDecl::FreeLex(role, arg()?.variant()?),
        "Numeral" => SymbolDecl::Numeral(role, numeral(arg()?)?),
        "Hosted" => SymbolDecl::Hosted(role, arg()?.ident()?, arg()?.variant()?),
        "Attached" => SymbolDecl::Attached(role, arg()?.ident()?, arg()?.variant()?),
        "Selected" => {
            let category = arg()?.ident()?;
            SymbolDecl::Selected(
                role,
                category,
                args.by_ref().map(Value::ident).collect::<Result<_, _>>()?,
            )
        }
        _ => return fail(at, format!("unknown symbol kind `{kind}`")),
    };
    if let Some(extra) = args.next() {
        return fail(extra.at, format!("`{kind}` takes no more arguments"));
    }
    Ok(decl)
}

fn numeral(value: Value) -> Result<sylex_lexicon::Numeral, LoadError> {
    let at = value.at;
    match value.kind {
        ValueKind::Call(name, args) if name == "Arabic" => match <[Value; 1]>::try_from(args) {
            Ok([grouped]) => Ok(sylex_lexicon::Numeral::Arabic(grouped.bool()?)),
            Err(_) => fail(at, "`Arabic` takes `true` or `false`"),
        },
        kind => Value { kind, at }.variant(),
    }
}

fn schema(kind: &str, item: &mut Item) -> Result<SchemaDecl, LoadError> {
    let name = item.name();
    let mut f = item.fields()?;
    let schema = match kind {
        "adjunct" => SchemaDecl::Adjunct {
            host: f.required("host")?.ident()?,
            adjunct: name,
            side: f.required("side")?.variant()?,
            host_kind: f.required("host_kind")?.variant()?,
            delimiter: f.char("delimiter")?,
            cost: f.cost(default_cost())?,
            equations: item.equations()?,
            nonfinal: f.flag("nonfinal")?,
        },
        "premodification" => SchemaDecl::Premodification {
            head: f.required("head")?.variant()?,
            modifier: name,
            host: f.required("host")?.ident()?,
            forms: f
                .required("forms")?
                .list()?
                .into_iter()
                .map(Value::variant)
                .collect::<Result<_, _>>()?,
            prefix: f.flag("prefix")?,
            equations: f.pairs("equations", "->", Value::variant, Value::equations)?,
        },
        "parameters" => SchemaDecl::Parameters {
            head: f.required("head")?.variant()?,
            phrase: name,
            binding: f.value("binding")?.map(Value::variant).transpose()?,
            coordinators: f.ident("coordinators")?,
            remnant: f.ident("remnant")?,
        },
        "valence" => valence(name, &mut f)?,
        "coordination" => coordination(name, &mut f, item.equations()?)?,
        _ => {
            return fail(
                item.at,
                format!("unknown keyword `{kind}`; a Production's name is capitalized"),
            );
        }
    };
    f.finish()?;
    Ok(schema)
}

fn valence(phrase: Ident, f: &mut Fields) -> Result<SchemaDecl, LoadError> {
    Ok(SchemaDecl::Valence {
        head: f.required("head")?.variant()?,
        phrase,
        cost: f.cost(0)?,
        complement_kind: f.ident("complement_kind")?,
        active: f.equations("active")?,
        passive: f.ident("passive")?,
        verbal_passive: f.equations("verbal_passive")?,
        prepositional_passive: f.equations("prepositional_passive")?,
        internalised: f.equations("internalised")?,
        adjectival: f.ident("adjectival")?,
        adjectival_passive: f.equations("adjectival_passive")?,
        interleaved: f.ident("interleaved")?,
        interleaved_internalised: f.ident("interleaved_internalised")?,
        heavy_shift: f.flag("heavy_shift")?,
        shifted_object: f.equations("shifted_object")?,
        postposed: f.ident("postposed")?,
        obligatory: f.flag("obligatory")?,
        agree: f.pairs("agree", "=", Value::ident, Value::ident)?,
        clusters: f.value("clusters")?.map(clusters).transpose()?,
        remnants: f.pairs("remnants", "->", Value::variant, Value::ident)?,
    })
}

fn clusters(value: Value) -> Result<ClustersDecl, LoadError> {
    let mut f = value.props()?;
    let adjunct_clusters = match f.value("adjunct_clusters")? {
        None => None,
        Some(value) => {
            let mut a = value.props()?;
            let decl = AdjunctClustersDecl {
                cluster: a.required("cluster")?.ident()?,
                coordination: a.required("coordination")?.ident()?,
                preposition: a.required("preposition")?.ident()?,
                remnant: a.required("remnant")?.ident()?,
            };
            a.finish()?;
            Some(decl)
        }
    };
    let decl = ClustersDecl {
        cluster: f.required("cluster")?.ident()?,
        coordination: f.required("coordination")?.ident()?,
        adjunct: f.ident("adjunct")?,
        remnants: f.pairs("remnants", "->", Value::variant, Value::ident)?,
        adjunct_clusters,
    };
    f.finish()?;
    Ok(decl)
}

fn coordination(
    category: Ident,
    f: &mut Fields,
    equations: Vec<Equation>,
) -> Result<SchemaDecl, LoadError> {
    let serial = f.value("serial")?.map(Value::bool).transpose()?;
    Ok(SchemaDecl::Coordination {
        category,
        coordinators: f.ident("coordinators")?,
        link: f.ident("link")?,
        cost: f.cost(default_cost())?,
        unshared: f.idents("unshared")?,
        equations,
        proximate: f.idents("proximate")?,
        resolve: f.pairs("resolve", "=", Value::ident, Value::ident)?,
        delimiter: f.char("delimiter")?,
        delimited: f.flag("delimited")?,
        serial,
        conjunct: f.ident("conjunct")?,
        unlike: f.ident("unlike")?,
        remnant: f.ident("remnant")?,
        recovered: f.idents("recovered")?,
        parallel: f.flag("parallel")?,
    })
}
