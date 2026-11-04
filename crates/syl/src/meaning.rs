//! Whether two trees mean the same, the formatter's correctness check.

use crate::span::FileId;
use crate::span::Span;
use crate::tree::Entry;
use crate::tree::Field;
use crate::tree::File;
use crate::tree::Item;
use crate::tree::Name;
use crate::tree::Shape;
use crate::tree::Sort;
use crate::tree::Value;
use crate::tree::ValueKind;

/// Whether `a` and `b` are the same tree up to what formatting may change.
///
/// Spans are ignored, and so is the blank-line flag of the first item of
/// every sequence and of the file, since [`format`](crate::format()) never
/// prints a blank line there. Everything else must match: every node, every
/// comment in its position and text, and every other recorded blank line.
#[must_use]
pub fn same_meaning(a: &File, b: &File) -> bool {
    normalized(a) == normalized(b)
}

const NOWHERE: Span = Span {
    file: FileId(0),
    start: 0,
    end: 0,
};

/// `file` with what formatting may change removed: every span is cleared,
/// and the blank-line flag of the first item of every sequence (and of the
/// file) is dropped.
fn normalized(file: &File) -> File {
    let mut file = file.clone();
    file.span = NOWHERE;
    items(&mut file.items, |declaration| {
        name(&mut declaration.kind);
        declaration.span = NOWHERE;
        match &mut declaration.shape {
            Shape::Nameless(v) => value(v),
            Shape::Named(named) => {
                named.span = NOWHERE;
                name(&mut named.name);
                items(&mut named.params, |param| {
                    param.span = NOWHERE;
                    name(&mut param.name);
                    sort(&mut param.sort);
                    if let Some(default) = &mut param.default {
                        value(default);
                    }
                });
                if let Some(s) = &mut named.sort {
                    sort(s);
                }
                if let Some(constructor) = &mut named.constructor {
                    name(constructor);
                }
                if let Some(body) = &mut named.body {
                    body.span = NOWHERE;
                    fields(&mut body.fields);
                }
            }
        }
    });
    file
}

fn items<T>(items: &mut [Item<T>], mut node: impl FnMut(&mut T)) {
    if let Some(first) = items.first_mut() {
        first.blank_line_before = false;
    }
    for item in items {
        match &mut item.entry {
            Entry::Comment(comment) => comment.span = NOWHERE,
            Entry::Node(n) => node(n),
        }
    }
}

fn fields(list: &mut [Item<Field>]) {
    items(list, |field| {
        field.span = NOWHERE;
        name(&mut field.name);
        value(&mut field.value);
    });
}

fn name(name: &mut Name) {
    name.span = NOWHERE;
}

fn sort(sort_ref: &mut Sort) {
    sort_ref.span = NOWHERE;
    name(&mut sort_ref.head);
    for arg in &mut sort_ref.args {
        sort(arg);
    }
}

fn value(v: &mut Value) {
    v.span = NOWHERE;
    match &mut v.kind {
        ValueKind::String(_) | ValueKind::Integer(_) => {}
        ValueKind::Parameter(n) => name(n),
        ValueKind::List(list) => items(list, value),
        ValueKind::Record(record) => {
            record.span = NOWHERE;
            fields(&mut record.fields);
        }
        ValueKind::Call(call) => {
            call.span = NOWHERE;
            name(&mut call.head);
            items(&mut call.args, |arg| {
                arg.span = NOWHERE;
                if let Some(n) = &mut arg.name {
                    name(n);
                }
                value(&mut arg.value);
            });
        }
    }
}
