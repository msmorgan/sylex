//! The declaration rules that need every declaration of a corpus at once
//! (`docs/decisions/syl.md` §1 and §3).
//!
//! Parsing alone refuses a capitalized word outside a sort or a constructor
//! clause. [`check`] refuses, across a set of bundles:
//!
//! - a primitive without a sort or a constructor clause, and a constructor
//!   clause on any other kind of declaration;
//! - two primitives for one constructor, that is, for the same constructor name
//!   at the same result sort;
//! - a word that names no declaration. A word is checked by name only: which
//!   declaration of that name it reaches depends on the expected sort, which
//!   this crate does not know. `none` is the language's own word for an omitted
//!   optional value and names no declaration.
//!
//! Words inside the metadata fields `lexical` and `spelling` are frames and
//! spelling, not meaning, so they are not checked.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use dpsi::Ident;

use crate::bundle::Bundle;
use crate::span::Span;
use crate::tree::Declaration;
use crate::tree::Field;
use crate::tree::Item;
use crate::tree::METADATA_FIELDS;
use crate::tree::Shape;
use crate::tree::Sort;
use crate::tree::Value;
use crate::tree::ValueKind;

/// The kind word of the one declaration that names a constructor.
const PRIMITIVE: &str = "primitive";

/// The word for an omitted optional value.
const NONE: &str = "none";

/// One broken rule, located by its span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub span: Span,
    pub kind: RefusalKind,
}

/// Which rule a declaration or word breaks.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusalKind {
    #[error("primitive `{0}` has no constructor clause (`= Constructor`)")]
    MissingConstructor(String),
    #[error("primitive `{0}` has no result sort")]
    MissingSort(String),
    #[error("a primitive is written with a name")]
    NamelessPrimitive,
    #[error("only a primitive names a constructor; `{declaration}` names `{constructor}`")]
    ConstructorOutsidePrimitive {
        /// The kind word and, when the header writes one, the name.
        declaration: String,
        constructor: String,
    },
    #[error("primitives `{first}` and `{second}` both name constructor `{constructor}` at {sort}")]
    DuplicateConstructor {
        first: String,
        second: String,
        constructor: String,
        sort: String,
    },
    #[error("word `{0}` names no declaration")]
    UnknownWord(String),
}

/// Why a set of bundles was refused: every refusal, each also rendered as
/// `path:line:column: message`.
#[derive(Debug, thiserror::Error)]
#[error("{}", rendered.join("\n"))]
pub struct CheckError {
    pub refusals: Vec<Refusal>,
    pub rendered: Vec<String>,
}

/// Checks the declaration rules across `bundles`, which together must hold
/// every declaration their words name.
///
/// # Errors
///
/// Every refusal, in file order.
///
/// # Panics
///
/// If a refusal's span lies in no bundle of `bundles`.
pub fn check(bundles: &[&Bundle]) -> Result<(), CheckError> {
    let declarations = || {
        bundles
            .iter()
            .flat_map(|bundle| bundle.files().values())
            .flat_map(|file| file.ast.declarations())
    };
    let declared: BTreeSet<Ident> = declarations()
        .filter_map(|declaration| match &declaration.shape {
            Shape::Header(header) => header.name.map(|name| name.ident),
            Shape::Value(_) => None,
        })
        .collect();
    let mut refusals = Vec::new();
    let mut constructors = BTreeMap::new();
    for declaration in declarations() {
        constructor_clause(declaration, &mut constructors, &mut refusals);
        let mut words = Words {
            declared: &declared,
            refusals: &mut refusals,
        };
        match &declaration.shape {
            Shape::Value(value) => words.value(value),
            Shape::Header(header) => {
                for param in header.params.iter().filter_map(Item::node) {
                    if let Some(default) = &param.default {
                        words.value(default);
                    }
                }
                if let Some(body) = &header.body {
                    words.fields(&body.fields);
                }
            }
        }
    }
    if refusals.is_empty() {
        return Ok(());
    }
    let rendered = refusals
        .iter()
        .map(|refusal| {
            let file = bundles
                .iter()
                .find_map(|bundle| bundle.file(refusal.span.file))
                .expect("every refusal lies in a checked bundle");
            format!("{}: {}", file.location(refusal.span), refusal.kind)
        })
        .collect();
    Err(CheckError { refusals, rendered })
}

/// Checks a declaration's constructor clause, recording a primitive's
/// constructor in `constructors`, keyed by result sort and constructor name.
fn constructor_clause(
    declaration: &Declaration,
    constructors: &mut BTreeMap<(String, Ident), Ident>,
    refusals: &mut Vec<Refusal>,
) {
    let kind = declaration.kind.ident;
    let header = match &declaration.shape {
        Shape::Header(header) => header,
        Shape::Value(_) => {
            if kind == PRIMITIVE {
                refusals.push(Refusal {
                    span: declaration.span,
                    kind: RefusalKind::NamelessPrimitive,
                });
            }
            return;
        }
    };
    if kind != PRIMITIVE {
        if let Some(constructor) = header.constructor {
            let declaration = match header.name {
                Some(name) => format!("{kind} {}", name.ident),
                None => kind.as_str().to_owned(),
            };
            refusals.push(Refusal {
                span: constructor.span,
                kind: RefusalKind::ConstructorOutsidePrimitive {
                    declaration,
                    constructor: constructor.ident.as_str().to_owned(),
                },
            });
        }
        return;
    }
    let Some(name) = header.name else {
        refusals.push(Refusal {
            span: declaration.span,
            kind: RefusalKind::NamelessPrimitive,
        });
        return;
    };
    let name = name.ident;
    let Some(sort) = &header.sort else {
        refusals.push(Refusal {
            span: header.span,
            kind: RefusalKind::MissingSort(name.as_str().to_owned()),
        });
        return;
    };
    let Some(constructor) = header.constructor else {
        refusals.push(Refusal {
            span: header.span,
            kind: RefusalKind::MissingConstructor(name.as_str().to_owned()),
        });
        return;
    };
    let sort = sort_text(sort);
    if let Some(first) = constructors.get(&(sort.clone(), constructor.ident)) {
        refusals.push(Refusal {
            span: constructor.span,
            kind: RefusalKind::DuplicateConstructor {
                first: first.as_str().to_owned(),
                second: name.as_str().to_owned(),
                constructor: constructor.ident.as_str().to_owned(),
                sort,
            },
        });
    } else {
        constructors.insert((sort, constructor.ident), name);
    }
}

/// A sort as written, `Option(List(Cost))`.
fn sort_text(sort: &Sort) -> String {
    let head = sort.head.ident.as_str();
    if sort.args.is_empty() {
        return head.to_owned();
    }
    let args: Vec<String> = sort.args.iter().map(sort_text).collect();
    format!("{head}({})", args.join(", "))
}

/// Refuses every word, outside metadata, that names no declaration.
struct Words<'a> {
    declared: &'a BTreeSet<Ident>,
    refusals: &'a mut Vec<Refusal>,
}

impl Words<'_> {
    fn fields(&mut self, fields: &[Item<Field>]) {
        for field in fields.iter().filter_map(Item::node) {
            if !METADATA_FIELDS.contains(&field.name.ident.as_str()) {
                self.value(&field.value);
            }
        }
    }

    fn value(&mut self, value: &Value) {
        match &value.kind {
            ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => {}
            ValueKind::List(values) => {
                for value in values.iter().filter_map(Item::node) {
                    self.value(value);
                }
            }
            ValueKind::Record(record) => self.fields(&record.fields),
            ValueKind::Call(call) => {
                let head = call.head.ident;
                let omitted = head == NONE && call.is_bare();
                if !omitted && !self.declared.contains(&head) {
                    self.refusals.push(Refusal {
                        span: call.head.span,
                        kind: RefusalKind::UnknownWord(head.as_str().to_owned()),
                    });
                }
                for arg in call.args.iter().filter_map(Item::node) {
                    self.value(&arg.value);
                }
            }
        }
    }
}
