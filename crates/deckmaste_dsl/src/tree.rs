//! The syntax tree.
//!
//! Comments are not attached to nodes. Each comma-separated sequence (list
//! items, record fields, call arguments, parameters) and the file's
//! declarations hold [`Item`]s, and an item is either a comment or a node. A
//! comment written where no item starts or ends (inside a header, a sort, or
//! between a field's `=` and its value) is moved to just before the item that
//! contains it.

use dpsi::Ident;

use crate::span::Span;

/// A parsed `.syl` file: its declarations and comments, in source order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub items: Vec<Item<Declaration>>,
    pub span: Span,
}

impl File {
    /// The declarations, without the comments.
    pub fn declarations(&self) -> impl Iterator<Item = &Declaration> {
        self.items.iter().filter_map(Item::node)
    }

    /// The doc comment of the declaration at `items[index]`: the run of
    /// comments written directly before it, with no blank line inside the run
    /// or between the run and the declaration. Empty when there is none, or
    /// when `items[index]` is not a declaration.
    ///
    /// Comments written inside the declaration, which the parser moved before
    /// it, are not part of the doc comment.
    #[must_use]
    pub fn doc_comment(&self, index: usize) -> Vec<&Comment> {
        let Some(Entry::Node(declaration)) = self.items.get(index).map(|item| &item.entry) else {
            return Vec::new();
        };
        // Skip the comments moved out of the declaration. The first of them
        // carries the declaration's blank-line flag.
        let mut first = index;
        while first > 0
            && self.items[first - 1]
                .comment()
                .is_some_and(|c| c.span.start > declaration.span.start)
        {
            first -= 1;
        }
        if self.items[first].blank_line_before {
            return Vec::new();
        }
        let mut doc = Vec::new();
        for item in self.items[..first].iter().rev() {
            let Entry::Comment(comment) = &item.entry else {
                break;
            };
            doc.push(comment);
            if item.blank_line_before {
                break;
            }
        }
        doc.reverse();
        doc
    }
}

/// One element of a sequence: a comment or a node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item<T> {
    /// Whether a blank line precedes the item in the source.
    pub blank_line_before: bool,
    pub entry: Entry<T>,
}

impl<T> Item<T> {
    /// The node, if this item is not a comment.
    pub fn node(&self) -> Option<&T> {
        match &self.entry {
            Entry::Node(node) => Some(node),
            Entry::Comment(_) => None,
        }
    }

    /// The comment, if this item is one.
    pub fn comment(&self) -> Option<&Comment> {
        match &self.entry {
            Entry::Comment(comment) => Some(comment),
            Entry::Node(_) => None,
        }
    }
}

/// What an [`Item`] holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry<T> {
    Comment(Comment),
    Node(T),
}

/// A `#` comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// Everything after the `#` up to the end of the line, verbatim.
    pub text: String,
    pub span: Span,
}

/// A word with its span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Name {
    pub ident: Ident,
    pub span: Span,
}

/// A declaration: its kind word, followed by either a named header or a single
/// value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    /// The first word, such as `primitive`, `template` or `card`. The parser
    /// gives no kind a special meaning.
    pub kind: Name,
    pub shape: Shape,
    pub span: Span,
}

/// The two declaration shapes, told apart by the tokens after the kind word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    /// `kind name(params): Sort = .constructor { fields }`, every part after
    /// the name optional.
    Named(NamedShape),
    /// `kind value`, such as `card { ... }` or `card split({ ... })`.
    Nameless(Value),
}

/// The parts of a named declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedShape {
    pub name: Name,
    /// Empty when no parameter list is written.
    pub params: Vec<Item<Param>>,
    /// The sort after `:`.
    pub sort: Option<Sort>,
    /// The constructor after `= .`, without the dot.
    pub constructor: Option<Name>,
    /// The `{ ... }` block.
    pub body: Option<Record>,
    /// From the name through the last part written.
    pub span: Span,
}

/// A header parameter, `name: Sort` or `name: Sort = default`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: Name,
    pub sort: Sort,
    pub default: Option<Value>,
    pub span: Span,
}

/// A sort reference, written like a call: `Option(List(Cost))`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sort {
    pub head: Name,
    /// Empty when no arguments are written.
    pub args: Vec<Sort>,
    pub span: Span,
}

/// A value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    pub kind: ValueKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    /// A string literal, unescaped.
    String(String),
    Integer(i64),
    /// `[a, b]`.
    List(Vec<Item<Value>>),
    /// `{ name = value, ... }`.
    Record(Record),
    /// `head(args)`, or a bare word `head`.
    Call(Call),
}

/// `{ name = value, ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub fields: Vec<Item<Field>>,
    pub span: Span,
}

impl Record {
    /// The fields, without the comments.
    pub fn nodes(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter().filter_map(Item::node)
    }
}

/// A record field, `name = value`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Name,
    pub value: Value,
    pub span: Span,
}

/// `head(arg, name = arg)`; a bare word is a call with no arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub head: Name,
    /// The arguments in written order; empty for a bare word.
    pub args: Vec<Item<Arg>>,
    pub span: Span,
}

impl Call {
    /// Whether this is a bare word, written without parentheses.
    #[must_use]
    pub fn is_bare(&self) -> bool {
        self.args.is_empty()
    }
}

/// A call argument, positional or `name = value`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub name: Option<Name>,
    pub value: Value,
    pub span: Span,
}
