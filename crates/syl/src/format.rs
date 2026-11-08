//! Prints a [`File`] in a canonical layout.
//!
//! The default [`Style`] is the one chosen for the `plugins` corpus:
//!
//! - A node goes on one line if it fits in [`Style::width`] columns, counting
//!   the closers and comma that follow it on the same line. An unbreakable
//!   string may run past the width.
//! - Otherwise every child goes on its own line, indented two spaces past the
//!   line that opens the node, starting on the line after the opener. Outer
//!   nodes break first.
//! - Inner closers hug the last child (`...)]))`); a declaration body's `}`
//!   goes on its own line. A sequence whose last item is a comment always ends
//!   with its closer on its own line.
//! - A list or record that is a call's first argument, unnamed, hugs the call's
//!   `(`, as in `and([`, unless hugging takes more lines. Only the opener
//!   moves: everything else keeps its unhugged place. See
//!   [`Style::hug_first_list`] for the exact rule.
//! - Sorts are never broken.
//! - Comments print verbatim on their own line at their sequence's indent. A
//!   comment or a recorded blank line inside a sequence forces it to break.
//! - A recorded blank line is kept as one blank line, except before the first
//!   item of a sequence or file, where it is dropped. The file ends with
//!   exactly one newline (an empty file prints as nothing).

use std::fmt::Write as _;

use crate::tree::Arg;
use crate::tree::Declaration;
use crate::tree::Entry;
use crate::tree::Field;
use crate::tree::File;
use crate::tree::Item;
use crate::tree::Param;
use crate::tree::Record;
use crate::tree::Shape;
use crate::tree::Sort;
use crate::tree::Value;
use crate::tree::ValueKind;

/// The block indent, in spaces.
const INDENT: usize = 2;

/// The printer's layout choices. [`Style::default`] is the corpus style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    /// The line width a node must fit in to stay on one line.
    pub width: usize,
    /// Where a broken inner node's closer goes. Declaration bodies always
    /// close on their own line.
    pub closers: Closers,
    /// Where a comma follows the last item of a broken sequence.
    pub trailing_commas: TrailingCommas,
    /// Whether a flat, non-empty record has spaces inside its braces:
    /// `{ a = b }` rather than `{a = b}`.
    pub record_spaces: bool,
    /// Whether a list or record that is a broken call's first argument, when
    /// that argument is unnamed, hugs the call's `(`: `and([` on one line.
    ///
    /// Hugging only moves the opener up; indentation still tracks nesting
    /// depth. The hugged layout is the unhugged one with the list broken and
    /// the line break between `head(` and `[` removed: the items stay two
    /// levels past the call's line, and the list's closer stays where the
    /// unhugged one goes. With hugged closers that is right after the last
    /// item, followed by the call's `)` when the list is its last argument
    /// (`])`) or by `,` and the remaining arguments, one level past the call's
    /// line. With closers on their own line, the list's `]` gets its own line
    /// one level past the call's line, and the call's `)` its own line at the
    /// call's indent. A named argument (`f(x = [`) and a list after the first
    /// argument never hug.
    ///
    /// The unhugged layout is used instead when it takes strictly fewer
    /// lines. Both layouts share the call's first line and everything after
    /// the list. If the list breaks unhugged too, hugging saves the line of
    /// its `[`, so hugging is shorter. Otherwise the unhugged list takes one
    /// line, flat one level in, followed by what follows it there (a comma,
    /// or for a last argument the call's closers when they hug, or the
    /// trailing comma, if any, when they do not). Hugged, each item takes a
    /// line of its own two levels in, at most as wide as the flat list
    /// (which has the extra `[` and `]`), and the `]` takes one more line when
    /// closers go on their own line. Unhugged is therefore shorter exactly
    /// when the list fits flat on its own line and either has two or more
    /// items or closers go on their own line; that is the test the printer
    /// applies. A one-item list under hugged closers ties, and hugs.
    pub hug_first_list: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: 100,
            closers: Closers::Hugged,
            trailing_commas: TrailingCommas::Never,
            record_spaces: false,
            hug_first_list: true,
        }
    }
}

/// Where a broken inner node's closer goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Closers {
    /// Right after the last child: `...)]))`.
    Hugged,
    /// On its own line, at the indent of the line that opened the node.
    OwnLine,
}

/// Where a broken sequence's last item is followed by a comma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrailingCommas {
    /// Never.
    Never,
    /// In declaration bodies only.
    Bodies,
    /// Wherever the sequence's closer goes on its own line.
    OwnLine,
}

/// Prints `file` in `style`.
#[must_use]
pub fn format(file: &File, style: &Style) -> String {
    let mut printer = Printer {
        style,
        out: String::new(),
        col: 0,
    };
    for (index, item) in file.items.iter().enumerate() {
        if index > 0 {
            printer.out.push('\n');
            if item.blank_line_before {
                printer.out.push('\n');
            }
            printer.col = 0;
        }
        match &item.entry {
            Entry::Comment(comment) => printer.text(&format!("#{}", comment.text)),
            Entry::Node(declaration) => printer.declaration(declaration),
        }
    }
    if !file.items.is_empty() {
        printer.out.push('\n');
    }
    printer.out
}

/// A layout node: either unbreakable text or a delimited sequence.
enum Layout {
    Text(String),
    Seq(Seq),
}

/// A non-empty delimited sequence.
struct Seq {
    kind: SeqKind,
    /// Everything up to and including the opening delimiter, such as `and(`.
    open: String,
    items: Vec<Element>,
    /// The one-line rendering; `None` when a comment or an interior blank line
    /// forces a break.
    flat: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeqKind {
    /// A call's arguments.
    Call,
    List,
    Record,
    /// A declaration's parameter list.
    Params,
}

impl SeqKind {
    fn close(self) -> &'static str {
        match self {
            SeqKind::Call | SeqKind::Params => ")",
            SeqKind::List => "]",
            SeqKind::Record => "}",
        }
    }
}

impl Seq {
    /// The list or record argument this call may hug: its first item, when
    /// that is an unnamed list or record argument.
    fn huggable(&self) -> Option<&Seq> {
        match self.items.first() {
            Some(Element {
                kind:
                    ElementKind::Node {
                        prefix,
                        layout: Layout::Seq(inner),
                    },
                ..
            }) if self.kind == SeqKind::Call
                && prefix.is_empty()
                && matches!(inner.kind, SeqKind::List | SeqKind::Record) =>
            {
                Some(inner)
            }
            _ => None,
        }
    }
}

struct Element {
    blank_line_before: bool,
    kind: ElementKind,
}

enum ElementKind {
    Comment(String),
    /// A node, printed after `prefix` (such as `name = `).
    Node {
        prefix: String,
        layout: Layout,
    },
}

impl Layout {
    fn flat(&self) -> Option<&str> {
        match self {
            Layout::Text(text) => Some(text),
            Layout::Seq(seq) => seq.flat.as_deref(),
        }
    }
}

/// Builds the layout nodes for one tree, under one style.
struct Builder<'s> {
    style: &'s Style,
}

impl Builder<'_> {
    fn seq(&self, kind: SeqKind, open: String, items: Vec<Element>) -> Layout {
        let flat = (|| {
            let mut parts = Vec::with_capacity(items.len());
            for (index, element) in items.iter().enumerate() {
                if index > 0 && element.blank_line_before {
                    return None;
                }
                let ElementKind::Node { prefix, layout } = &element.kind else {
                    return None;
                };
                parts.push(format!("{prefix}{}", layout.flat()?));
            }
            let pad = if kind == SeqKind::Record && self.style.record_spaces { " " } else { "" };
            Some(format!(
                "{open}{pad}{}{pad}{}",
                parts.join(", "),
                kind.close()
            ))
        })();
        Layout::Seq(Seq {
            kind,
            open,
            items,
            flat,
        })
    }

    fn value(&self, value: &Value) -> Layout {
        match &value.kind {
            ValueKind::String(text) => Layout::Text(string(text)),
            ValueKind::Integer(n) => Layout::Text(n.to_string()),
            ValueKind::Parameter(name) => Layout::Text(format!("${}", name.ident)),
            ValueKind::List(items) if items.is_empty() => Layout::Text("[]".to_owned()),
            ValueKind::List(items) => {
                let elements = elements(items, |v| (String::new(), self.value(v)));
                self.seq(SeqKind::List, "[".to_owned(), elements)
            }
            ValueKind::Record(record) => self.record(record),
            ValueKind::Call(call) if call.is_bare() => Layout::Text(call.head.ident.to_string()),
            ValueKind::Call(call) => {
                let elements = elements(&call.args, |arg| self.arg(arg));
                self.seq(SeqKind::Call, format!("{}(", call.head.ident), elements)
            }
        }
    }

    fn record(&self, record: &Record) -> Layout {
        if record.fields.is_empty() {
            return Layout::Text("{}".to_owned());
        }
        let elements = elements(&record.fields, |field| self.field(field));
        self.seq(SeqKind::Record, "{".to_owned(), elements)
    }

    fn field(&self, field: &Field) -> (String, Layout) {
        (format!("{} = ", field.name.ident), self.value(&field.value))
    }

    fn arg(&self, arg: &Arg) -> (String, Layout) {
        let prefix = arg
            .name
            .map_or_else(String::new, |name| format!("{} = ", name.ident));
        (prefix, self.value(&arg.value))
    }

    fn param(&self, param: &Param) -> (String, Layout) {
        let head = format!("{}: {}", param.name.ident, sort(&param.sort));
        match &param.default {
            None => (String::new(), Layout::Text(head)),
            Some(default) => (format!("{head} = "), self.value(default)),
        }
    }
}

/// The layout elements of a sequence's items, `node` giving each node's
/// prefix and layout.
fn elements<T>(items: &[Item<T>], mut node: impl FnMut(&T) -> (String, Layout)) -> Vec<Element> {
    items
        .iter()
        .map(|item| Element {
            blank_line_before: item.blank_line_before,
            kind: match &item.entry {
                Entry::Comment(comment) => ElementKind::Comment(comment.text.clone()),
                Entry::Node(n) => {
                    let (prefix, layout) = node(n);
                    ElementKind::Node { prefix, layout }
                }
            },
        })
        .collect()
}

/// A string literal, with `"` and `\` escaped.
fn string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// A sort, always on one line.
fn sort(sort_ref: &Sort) -> String {
    let mut out = sort_ref.head.ident.to_string();
    if !sort_ref.args.is_empty() {
        let args: Vec<_> = sort_ref.args.iter().map(sort).collect();
        let _ = write!(out, "({})", args.join(", "));
    }
    out
}

fn width(text: &str) -> usize {
    text.chars().count()
}

struct Printer<'s> {
    style: &'s Style,
    out: String,
    /// The column the next character prints at, counted in characters.
    col: usize,
}

impl Printer<'_> {
    fn text(&mut self, text: &str) {
        self.out.push_str(text);
        // A string literal may hold a raw newline.
        match text.rfind('\n') {
            Some(at) => self.col = width(&text[at + 1..]),
            None => self.col += width(text),
        }
    }

    fn newline(&mut self, indent: usize) {
        self.out.push('\n');
        self.out.extend(std::iter::repeat_n(' ', indent));
        self.col = indent;
    }

    fn builder(&self) -> Builder<'_> {
        Builder { style: self.style }
    }

    fn declaration(&mut self, declaration: &Declaration) {
        self.text(&format!("{} ", declaration.kind.ident));
        match &declaration.shape {
            Shape::Nameless(Value {
                kind: ValueKind::Record(record),
                ..
            }) => self.body(record),
            Shape::Nameless(value) => {
                let layout = self.builder().value(value);
                self.layout(&layout, 0, 0);
            }
            Shape::Named(named) => {
                self.text(named.name.ident.as_str());
                let mut rest = String::new();
                if let Some(sort_ref) = &named.sort {
                    let _ = write!(rest, ": {}", sort(sort_ref));
                }
                if let Some(constructor) = named.constructor {
                    let _ = write!(rest, " = {}", constructor.ident);
                }
                if named.body.is_some() {
                    rest.push(' ');
                }
                if !named.params.is_empty() {
                    let builder = self.builder();
                    let elements = elements(&named.params, |p| builder.param(p));
                    let layout = builder.seq(SeqKind::Params, "(".to_owned(), elements);
                    let suffix = width(&rest) + usize::from(named.body.is_some());
                    self.layout(&layout, 0, suffix);
                }
                self.text(&rest);
                if let Some(body) = &named.body {
                    self.body(body);
                }
            }
        }
    }

    /// A declaration body: broken unless empty, its `}` on its own line.
    fn body(&mut self, record: &Record) {
        if record.fields.is_empty() {
            self.text("{}");
            return;
        }
        let builder = self.builder();
        let elements = elements(&record.fields, |f| builder.field(f));
        self.text("{");
        self.elements(&elements, 0, "}", 0, true, false);
    }

    /// Prints `layout` at the current column. `indent` is the indent of the
    /// current line; `suffix` is the width of what follows it on its last
    /// line.
    fn layout(&mut self, layout: &Layout, indent: usize, suffix: usize) {
        match layout {
            Layout::Text(text) => self.text(text),
            Layout::Seq(seq) => {
                if let Some(flat) = &seq.flat
                    && self.col + width(flat) + suffix <= self.style.width
                {
                    self.text(flat);
                } else {
                    self.broken(seq, indent, suffix);
                }
            }
        }
    }

    fn broken(&mut self, seq: &Seq, indent: usize, suffix: usize) {
        let hug = self.style.hug_first_list
            && seq
                .huggable()
                .is_some_and(|inner| !self.unhugged_is_shorter(seq, inner, indent, suffix));
        self.text(&seq.open);
        self.elements(&seq.items, indent, seq.kind.close(), suffix, false, hug);
    }

    /// Whether `inner`, the list or record `seq` may hug, takes fewer lines
    /// unhugged: it fits flat on its own line one level in, followed by what
    /// follows it there (see [`Printer::node_end`]), and either has two or
    /// more items or its closer would go on its own line when hugged. See
    /// [`Style::hug_first_list`].
    fn unhugged_is_shorter(&self, seq: &Seq, inner: &Seq, indent: usize, suffix: usize) -> bool {
        let (_, after) = self.node_end(&seq.items, 0, seq.kind.close(), suffix, false);
        let fits = inner
            .flat
            .as_ref()
            .is_some_and(|flat| indent + INDENT + width(flat) + after <= self.style.width);
        fits && (inner.items.len() >= 2 || self.style.closers == Closers::OwnLine)
    }

    /// Whether node `index` of a broken sequence takes a comma, and the width
    /// of what follows it on its line: its comma, or, for the last node, what
    /// [`Printer::last_item_suffix`] gives.
    fn node_end(
        &self,
        elements: &[Element],
        index: usize,
        close: &str,
        suffix: usize,
        body: bool,
    ) -> (bool, usize) {
        let last_node = elements
            .iter()
            .rposition(|e| matches!(e.kind, ElementKind::Node { .. }));
        let ends_with_comment = last_node.is_none_or(|last| last + 1 < elements.len());
        if Some(index) == last_node {
            let (_, trailing) = self.sequence_end(body, ends_with_comment);
            (
                trailing,
                self.last_item_suffix(close, suffix, body, ends_with_comment),
            )
        } else {
            (true, 1)
        }
    }

    /// Whether a broken sequence's closer goes on its own line, and whether its
    /// last node takes a trailing comma.
    fn sequence_end(&self, body: bool, ends_with_comment: bool) -> (bool, bool) {
        let closer_own_line = body || self.style.closers == Closers::OwnLine || ends_with_comment;
        let trailing = match self.style.trailing_commas {
            TrailingCommas::Never => false,
            TrailingCommas::Bodies => body,
            TrailingCommas::OwnLine => closer_own_line,
        };
        (closer_own_line, trailing)
    }

    /// The width of what follows a broken sequence's last node on its line:
    /// `close` and the `suffix` after it when the closer hugs, otherwise the
    /// trailing comma, if any.
    fn last_item_suffix(
        &self,
        close: &str,
        suffix: usize,
        body: bool,
        ends_with_comment: bool,
    ) -> usize {
        match self.sequence_end(body, ends_with_comment) {
            (false, _) => close.len() + suffix,
            (true, trailing) => usize::from(trailing),
        }
    }

    /// Prints a broken sequence's items, one per line one level past
    /// `indent`, then `close`. `suffix` is the width of what follows `close`
    /// on its line. With `hug`, the first item, a list or record, stays on
    /// the opener's line and breaks (see [`Style::hug_first_list`]).
    fn elements(
        &mut self,
        elements: &[Element],
        indent: usize,
        close: &str,
        suffix: usize,
        body: bool,
        hug: bool,
    ) {
        let inner = indent + INDENT;
        let last_node = elements
            .iter()
            .rposition(|e| matches!(e.kind, ElementKind::Node { .. }));
        let ends_with_comment = last_node.is_none_or(|last| last + 1 < elements.len());
        let (closer_own_line, _) = self.sequence_end(body, ends_with_comment);
        for (index, element) in elements.iter().enumerate() {
            let hugged = hug && index == 0;
            if index > 0 && element.blank_line_before {
                self.out.push('\n');
            }
            if !hugged {
                self.newline(inner);
            }
            match &element.kind {
                ElementKind::Comment(text) => self.text(&format!("#{text}")),
                ElementKind::Node { prefix, layout } => {
                    let (comma, after) = self.node_end(elements, index, close, suffix, body);
                    self.text(prefix);
                    match layout {
                        Layout::Seq(seq) if hugged => self.broken(seq, inner, after),
                        _ => self.layout(layout, inner, after),
                    }
                    if comma {
                        self.text(",");
                    }
                }
            }
        }
        if closer_own_line {
            self.newline(indent);
        }
        self.text(close);
    }
}
