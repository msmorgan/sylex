//! `cargo xtask dsl check`: every reference in the `.syl` bundles resolves.
//!
//! A temporary validator that guards `.syl` until the Rust resolution layer
//! (step 7 of the semantics-v3 plan) replaces it. It checks names, sorts and
//! argument binding only, following `docs/decisions/syl.md` §2 and §3; it
//! expands nothing and reads no meaning.
//!
//! [`check`] first runs [`sylex_syl::check()`] (constructor clauses, and words
//! that name no declaration at any sort), then resolves every value at its
//! expected sort against the `(name, sort)` signatures the declaration headers
//! declare. Declaration kinds, record sorts and reserved
//! constructors have no `.syl` declaration; their provisional schemas are
//! [`schema`]'s. The only words and sorts written here are the language's own
//! (`none`, `some`, `Option`, `List` and the literal sorts, `syl.md` §3).
//! The coercions are declared in `.syl`; [`coercion`] reads and checks them.

mod call;
mod coercion;
mod cycles;
mod refusal;
pub(crate) mod schema;
mod sort;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map::Entry;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Args;
use sylex_syl::Bundle;
use sylex_syl::Declaration;
use sylex_syl::FileId;
use sylex_syl::Item;
use sylex_syl::METADATA_FIELDS;
use sylex_syl::Record;
use sylex_syl::Shape;
use sylex_syl::Sort;
use sylex_syl::Span;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use self::coercion::Coercion;
pub use self::refusal::CheckError;
pub use self::refusal::Refusal;
pub use self::refusal::RefusalKind;
use self::schema::Form;
use self::schema::KINDS;
use self::schema::Kind;
use self::schema::RECORDS;
use self::sort::SortKey;
use self::sort::sort_key;

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Bundle directories, the builtin bundle first (default: `plugins/builtin`
    /// and `plugins/canon`).
    bundles: Vec<PathBuf>,
}

pub fn run(args: &CheckArgs) -> anyhow::Result<()> {
    let roots = if args.bundles.is_empty() {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../plugins")
            .canonicalize()?;
        vec![plugins.join("builtin"), plugins.join("canon")]
    } else {
        args.bundles.clone()
    };
    let mut bundles: Vec<Arc<Bundle>> = Vec::new();
    for root in &roots {
        let first = bundles.last().map_or(FileId(0), |b| b.next_file_id());
        bundles.push(Bundle::load(root, first)?);
    }
    let (builtin, others) = bundles.split_first().expect("at least one bundle");
    let others: Vec<&Bundle> = others.iter().map(AsRef::as_ref).collect();
    match check(builtin, &others) {
        Ok(summary) => {
            println!("dsl check: {summary}");
            Ok(())
        }
        Err(error) => {
            eprintln!("{error}");
            anyhow::bail!("dsl check: {} refused", error.count())
        }
    }
}

/// What a passing check covered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub files: usize,
    /// The declarations of each kind.
    pub declarations: BTreeMap<&'static str, usize>,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total: usize = self.declarations.values().sum();
        write!(f, "{} files, {total} declarations resolve (", self.files)?;
        for (index, (kind, count)) in self.declarations.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{kind} {count}")?;
        }
        f.write_str(")")
    }
}

/// A parameter of a declaration, or a field of a record or block.
#[derive(Debug, Clone)]
struct Slot {
    name: String,
    sort: SortKey,
    optional: bool,
    /// A parameter's declared default; `None` for a field.
    default: Option<Value>,
}

/// What a word resolves to at one sort.
#[derive(Debug)]
struct Signature {
    params: Vec<Slot>,
    /// The declaration's node in the dependency graph; `None` for a second
    /// registration and a catalog declaration, which are no dependency edge.
    node: Option<usize>,
    /// Declared under the Ring exception (`schema::RING_EXCEPTION`), so only
    /// the Ring's file may call it.
    ring_only: bool,
    /// The declaration's span.
    span: Span,
}

type Registry = BTreeMap<(String, SortKey), Signature>;

/// One declaration whose header passed.
struct Decl<'a> {
    declaration: &'a Declaration,
    kind: &'static Kind,
    /// `None` for a kind written without a name, which declares no word.
    name: Option<String>,
    sort: SortKey,
    params: Vec<Slot>,
    /// Whether this is the Ring exception's file.
    ring: bool,
}

impl Decl<'_> {
    fn block(&self) -> Option<&Record> {
        match &self.declaration.shape {
            Shape::Header(header) => header.body.as_ref(),
            Shape::Value(_) => None,
        }
    }

    /// The declaration as a refusal names it: its kind word and its name, or
    /// for a nameless header its signature.
    fn label(&self) -> String {
        let word = self.kind.word;
        if let Some(name) = &self.name {
            return format!("`{word}` {name}");
        }
        let params: Vec<String> = self.params.iter().map(|p| p.sort.to_string()).collect();
        format!("`{word}`({}): {}", params.join(", "), self.sort)
    }

    fn field(&self, name: &str) -> Option<&Value> {
        self.block()?
            .nodes()
            .find(|f| f.name.ident == name)
            .map(|f| &f.value)
    }
}

/// Checks that every reference in `builtin` and `others` resolves.
///
/// # Errors
///
/// `sylex_syl::check`'s refusals, or else every resolution refusal.
pub fn check(builtin: &Bundle, others: &[&Bundle]) -> Result<Summary, CheckError> {
    let bundles: Vec<&Bundle> = std::iter::once(builtin)
        .chain(others.iter().copied())
        .collect();
    sylex_syl::check(&bundles).map_err(CheckError::Declarations)?;
    let ring_file = builtin.root().join(schema::RING_EXCEPTION.0);
    let mut refusals = Vec::new();
    let mut decls = Vec::new();
    let mut summary = Summary {
        files: 0,
        declarations: BTreeMap::new(),
    };
    for bundle in &bundles {
        for file in bundle.files().values() {
            summary.files += 1;
            let ring = std::ptr::eq(*bundle, builtin) && file.path == ring_file;
            for declaration in file.ast.declarations() {
                match header(declaration, ring) {
                    Ok(decl) => {
                        *summary.declarations.entry(decl.kind.word).or_default() += 1;
                        decls.push(decl);
                    }
                    Err(refusal) => refusals.push(refusal),
                }
            }
        }
    }
    let registry = register(&decls, &mut refusals);
    unknown_sorts(&decls, &known_sorts(&decls, &registry), &mut refusals);
    let coercions = coercion::coercions(&decls, &registry, &mut refusals);
    let mut edges = vec![Vec::new(); decls.len()];
    for (decl, edges) in decls.iter().zip(&mut edges) {
        resolve(decl, &registry, &coercions, &mut refusals, edges);
    }
    for cycle in cycles::cycles(&edges) {
        // A cycle names a declaration by its name, a nameless one by its label.
        let name = |i: usize| match &decls[i].name {
            Some(name) => name.clone(),
            None => decls[i].label(),
        };
        refusals.push(Refusal {
            span: decls[cycle[0]].declaration.span,
            kind: RefusalKind::Cycle(cycle.into_iter().map(name).collect()),
        });
    }
    if refusals.is_empty() {
        return Ok(summary);
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
    Err(CheckError::Resolution { refusals, rendered })
}

/// Checks a declaration's kind, name, sort and parameters.
fn header(declaration: &Declaration, ring: bool) -> Result<Decl<'_>, Refusal> {
    let refuse = |span, kind| Err(Refusal { span, kind });
    let word = declaration.kind.ident.as_str();
    let Some(kind) = KINDS.iter().find(|kind| kind.word == word) else {
        return refuse(
            declaration.kind.span,
            RefusalKind::UnknownKind(word.to_owned()),
        );
    };
    let decl = |name, sort, params| Decl {
        declaration,
        kind,
        name,
        sort,
        params,
        ring,
    };
    let header = match (&declaration.shape, kind.form) {
        (Shape::Value(_), Form::Value) => {
            let sort = kind.sort.expect("a value kind implies its sort");
            return Ok(decl(None, SortKey::table(sort), Vec::new()));
        }
        (Shape::Header(header), Form::Block | Form::OptionalBlock) => match header.name {
            Some(_) => header,
            None => return refuse(declaration.span, RefusalKind::MissingName(word.to_owned())),
        },
        (Shape::Header(header), Form::NamelessBlock) => match header.name {
            Some(name) => return refuse(name.span, RefusalKind::WrittenName(word.to_owned())),
            None => header,
        },
        (Shape::Value(_), Form::Block | Form::OptionalBlock) => {
            return refuse(declaration.span, RefusalKind::MissingName(word.to_owned()));
        }
        (Shape::Value(_), Form::NamelessBlock) => {
            return refuse(
                declaration.span,
                RefusalKind::MissingHeader(word.to_owned()),
            );
        }
        (Shape::Header(header), Form::Value) => match header.name {
            Some(_) => return refuse(declaration.span, RefusalKind::WrittenName(word.to_owned())),
            None => {
                return refuse(
                    declaration.span,
                    RefusalKind::WrittenHeader(word.to_owned()),
                );
            }
        },
    };
    if let Some(name) = header.name
        && name.ident == "none"
    {
        return refuse(name.span, RefusalKind::NamedNone);
    }
    let name = header.name.map(|name| name.ident.as_str());
    let sort = match (kind.sort, &header.sort) {
        (None, Some(sort)) => sort_key(sort)?,
        (None, None) => return refuse(header.span, RefusalKind::MissingSort(word.to_owned())),
        (Some(implied), None) => SortKey::table(implied),
        (Some(implied), Some(sort)) => {
            return refuse(
                sort.span,
                RefusalKind::WrittenSort {
                    kind: word.to_owned(),
                    sort: implied.to_owned(),
                },
            );
        }
    };
    if header.body.is_none() && matches!(kind.form, Form::Block | Form::NamelessBlock) {
        return refuse(header.span, RefusalKind::MissingBlock(word.to_owned()));
    }
    let mut params: Vec<Slot> = Vec::new();
    for param in header.params.iter().filter_map(Item::node) {
        let param_name = param.name.ident.as_str();
        if params.iter().any(|p| p.name == param_name) {
            return refuse(
                param.name.span,
                RefusalKind::DuplicateParameter(param_name.to_owned()),
            );
        }
        params.push(Slot {
            name: param_name.to_owned(),
            sort: sort_key(&param.sort)?,
            optional: param.default.is_some(),
            default: param.default.clone(),
        });
    }
    Ok(decl(name.map(str::to_owned), sort, params))
}

/// Registers every named declaration at its `(name, sort)` and its second
/// sort, refusing duplicates and primitives that build a reserved
/// constructor.
fn register(decls: &[Decl<'_>], refusals: &mut Vec<Refusal>) -> Registry {
    let mut registry = Registry::new();
    let mut insert =
        |key: (String, SortKey), signature, span, refusals: &mut Vec<Refusal>| match registry
            .entry(key)
        {
            Entry::Occupied(entry) => refusals.push(Refusal {
                span,
                kind: RefusalKind::DuplicateDeclaration {
                    name: entry.key().0.clone(),
                    sort: entry.key().1.to_string(),
                },
            }),
            Entry::Vacant(entry) => {
                entry.insert(signature);
            }
        };
    for (index, decl) in decls.iter().enumerate() {
        let Some(name) = &decl.name else {
            continue;
        };
        let Shape::Header(header) = &decl.declaration.shape else {
            unreachable!("a named declaration has a header");
        };
        let mut ring_only = false;
        if let Some(constructor) = header.constructor
            && let Some(reserved) = schema::reserved(
                name,
                decl.params.len(),
                &decl.sort.to_string(),
                constructor.ident.as_str(),
            )
        {
            ring_only = decl.ring && reserved == schema::RING_EXCEPTION.1;
            if !ring_only {
                refusals.push(Refusal {
                    span: constructor.span,
                    kind: RefusalKind::ReservedConstructor(reserved),
                });
            }
        }
        let span = decl.declaration.span;
        let signature = Signature {
            params: decl.params.clone(),
            node: (!decl.kind.catalog).then_some(index),
            ring_only,
            span,
        };
        insert((name.clone(), decl.sort.clone()), signature, span, refusals);
        let Some(secondary) = decl.kind.secondary else {
            continue;
        };
        let unless = secondary.unless.iter().any(|(field, word)| {
            decl.field(field)
                .is_some_and(|value| word.is_none_or(|word| is_bare(value, word)))
        });
        if unless {
            continue;
        }
        let signature = Signature {
            params: Vec::new(),
            node: None,
            ring_only: false,
            span,
        };
        let key = (name.clone(), SortKey::named(secondary.sort));
        insert(key, signature, span, refusals);
    }
    registry
}

/// The sorts a literal stands at (`syl.md` §3).
const LITERAL_SORTS: [&str; 3] = ["String", "Int", "Nat"];

/// The sort names a written sort may use: the literal sorts, the sort
/// constructors and the undeclared sorts, and every name both produced and
/// consumed. A declaration's result sort (a coercion's included) or a record
/// sort produces a name; a parameter's sort, or a kind's sort or field sort, or
/// a record field's sort consumes one. A name only produced or only consumed is
/// a misspelling, or a sort no value can reach.
fn known_sorts(decls: &[Decl<'_>], registry: &Registry) -> BTreeSet<String> {
    fn add(sort: &SortKey, out: &mut BTreeSet<String>) {
        let mut heads = BTreeSet::new();
        sort.heads(&mut heads);
        out.extend(heads.into_iter().map(str::to_owned));
    }
    let mut produced = BTreeSet::new();
    for (_, sort) in registry.keys() {
        add(sort, &mut produced);
    }
    for decl in decls
        .iter()
        .filter(|decl| decl.kind.word == schema::COERCION)
    {
        add(&decl.sort, &mut produced);
    }
    produced.extend(RECORDS.iter().map(|record| record.sort.to_owned()));
    let mut consumed = BTreeSet::new();
    for param in decls.iter().flat_map(|decl| &decl.params) {
        add(&param.sort, &mut consumed);
    }
    for kind in KINDS {
        let secondary = kind.secondary.map(|secondary| secondary.sort);
        let fields = kind.fields.iter().filter_map(|field| field.sort);
        for sort in kind.sort.into_iter().chain(secondary).chain(fields) {
            add(&SortKey::table(sort), &mut consumed);
        }
    }
    for (_, sort, _) in RECORDS.iter().flat_map(|record| record.fields) {
        add(&SortKey::table(sort), &mut consumed);
    }
    LITERAL_SORTS
        .into_iter()
        .chain(schema::SORT_CONSTRUCTORS)
        .map(str::to_owned)
        .chain(produced.intersection(&consumed).cloned())
        .collect()
}

/// Refuses every written sort name `known` lacks.
fn unknown_sorts(decls: &[Decl<'_>], known: &BTreeSet<String>, refusals: &mut Vec<Refusal>) {
    for decl in decls {
        let Shape::Header(header) = &decl.declaration.shape else {
            continue;
        };
        let params = header.params.iter().filter_map(Item::node).map(|p| &p.sort);
        for sort in header.sort.iter().chain(params) {
            unknown_sort(sort, known, refusals);
        }
    }
}

fn unknown_sort(sort: &Sort, known: &BTreeSet<String>, refusals: &mut Vec<Refusal>) {
    let head = sort.head.ident.as_str();
    if !known.contains(head) {
        refusals.push(Refusal {
            span: sort.head.span,
            kind: RefusalKind::UnknownSort(head.to_owned()),
        });
    }
    for arg in &sort.args {
        unknown_sort(arg, known, refusals);
    }
}

fn is_bare(value: &Value, word: &str) -> bool {
    matches!(&value.kind, ValueKind::Call(call) if call.is_bare() && call.head.ident == word)
}

/// Resolves one declaration's parameter defaults and block or value.
fn resolve(
    decl: &Decl<'_>,
    registry: &Registry,
    coercions: &[Coercion],
    refusals: &mut Vec<Refusal>,
    edges: &mut Vec<usize>,
) {
    let ring = decl.ring;
    if let Shape::Header(header) = &decl.declaration.shape {
        // A default reads only the parameters before it.
        for (index, param) in header.params.iter().filter_map(Item::node).enumerate() {
            if let Some(default) = &param.default {
                let sort = &decl.params[index].sort;
                let params = &decl.params[..index];
                Resolver {
                    registry,
                    coercions,
                    refusals: &mut *refusals,
                    edges: &mut *edges,
                    params,
                    ring,
                }
                .value(default, sort);
            }
        }
    }
    let params = &decl.params;
    Resolver {
        registry,
        coercions,
        refusals,
        edges,
        params,
        ring,
    }
    .body(decl);
}

/// Resolves values for one declaration.
struct Resolver<'r> {
    registry: &'r Registry,
    coercions: &'r [Coercion],
    refusals: &'r mut Vec<Refusal>,
    /// The dependency nodes the declaration reaches.
    edges: &'r mut Vec<usize>,
    /// The parameters a `$name` may read.
    params: &'r [Slot],
    ring: bool,
}

impl Resolver<'_> {
    fn refuse(&mut self, span: Span, kind: RefusalKind) {
        self.refusals.push(Refusal { span, kind });
    }

    fn body(&mut self, decl: &Decl<'_>) {
        let header = match &decl.declaration.shape {
            Shape::Value(value) => return self.value(value, &decl.sort),
            Shape::Header(header) => header,
        };
        // Only a kind whose block is optional may omit it, and then there is
        // nothing to resolve.
        let Some(block) = &header.body else {
            return;
        };
        let at = decl.label();
        let slots: Vec<Slot> = decl
            .kind
            .fields
            .iter()
            .map(|spec| Slot {
                name: spec.name.to_owned(),
                sort: spec.sort.map_or_else(|| decl.sort.clone(), SortKey::table),
                optional: !spec.required,
                default: None,
            })
            .collect();
        self.fields(block, &slots, &at);
    }

    /// Resolves a record's or block's fields against `slots`, skipping the
    /// metadata fields.
    fn fields(&mut self, record: &Record, slots: &[Slot], at: &str) {
        let mut written = BTreeSet::new();
        for field in record.nodes() {
            let name = field.name.ident.as_str();
            if !written.insert(name) {
                self.refuse(
                    field.name.span,
                    RefusalKind::DuplicateField(name.to_owned()),
                );
                continue;
            }
            if METADATA_FIELDS.contains(&name) {
                continue;
            }
            match slots.iter().find(|slot| slot.name == name) {
                Some(slot) => self.value(&field.value, &slot.sort),
                None => self.refuse(
                    field.name.span,
                    RefusalKind::UnknownField {
                        field: name.to_owned(),
                        at: at.to_owned(),
                    },
                ),
            }
        }
        for slot in slots {
            if !slot.optional && !written.contains(slot.name.as_str()) {
                self.refuse(
                    record.span,
                    RefusalKind::MissingField {
                        field: slot.name.clone(),
                        at: at.to_owned(),
                    },
                );
            }
        }
    }

    fn record(&mut self, record: &Record, sort: &SortKey) {
        let Some(spec) = RECORDS
            .iter()
            .find(|spec| sort.args.is_empty() && spec.sort == sort.head)
        else {
            self.refuse(record.span, RefusalKind::RecordAt(sort.to_string()));
            return;
        };
        let slots: Vec<Slot> = spec
            .fields
            .iter()
            .map(|(name, sort, optional)| Slot {
                name: (*name).to_owned(),
                sort: SortKey::table(sort),
                optional: *optional,
                default: None,
            })
            .collect();
        self.fields(record, &slots, spec.sort);
    }

    /// Resolves `value` at `expected` (`syl.md` §3).
    fn value(&mut self, value: &Value, expected: &SortKey) {
        self.value_at(value, expected, true);
    }

    /// Resolves `value` at `expected`, trying a coercion into `expected` at a
    /// sort mismatch when `coerce` holds. A value a coercion reached is
    /// resolved with `coerce` false: a coercion is one hop.
    fn value_at(&mut self, value: &Value, expected: &SortKey, coerce: bool) {
        match &value.kind {
            ValueKind::Parameter(name) => {
                return self.parameter(name.ident.as_str(), value.span, expected, coerce);
            }
            ValueKind::Call(call) => {
                let head = call.head.ident.as_str();
                if head == "none" {
                    if expected.option_inner().is_none() || !call.is_bare() {
                        self.refuse(
                            value.span,
                            RefusalKind::NoneOutsideOption(expected.to_string()),
                        );
                    }
                    return;
                }
                if head == "some" && expected.option_inner().is_some() {
                    return self.refuse(value.span, RefusalKind::SomeWritten);
                }
                if let Some(signature) = self.registry.get(&(head.to_owned(), expected.clone())) {
                    return self.call(call, signature);
                }
            }
            ValueKind::String(_)
            | ValueKind::Integer(_)
            | ValueKind::List(_)
            | ValueKind::Record(_) => {}
        }
        // A `T` at `Option(T)`.
        if let Some(inner) = expected.option_inner() {
            return self.value_at(value, inner, coerce);
        }
        if coerce && !self.stands_at(value, expected) && self.coerce(value, expected) {
            return;
        }
        let at = expected.to_string();
        match &value.kind {
            ValueKind::String(_) => {
                if expected.head != "String" {
                    self.refuse(value.span, RefusalKind::StringAt(at));
                }
            }
            ValueKind::Integer(number) => match expected.head.as_str() {
                "Nat" if *number < 0 => self.refuse(value.span, RefusalKind::NegativeAtNat),
                "Int" | "Nat" => {}
                _ => self.refuse(value.span, RefusalKind::IntegerAt(at)),
            },
            ValueKind::List(items) => match expected.list_inner() {
                Some(inner) => {
                    for item in items.iter().filter_map(Item::node) {
                        self.value(item, inner);
                    }
                }
                None => self.refuse(value.span, RefusalKind::ListAt(at)),
            },
            ValueKind::Record(record) => self.record(record, expected),
            ValueKind::Call(call) => self.refuse(
                call.head.span,
                RefusalKind::UnknownWord {
                    word: call.head.ident.as_str().to_owned(),
                    sort: at,
                },
            ),
            ValueKind::Parameter(_) => unreachable!("a parameter is resolved above"),
        }
    }

    /// Whether `value`, not a parameter, stands at `sort` itself, with no
    /// coercion: a word declared there, or a literal at its own sort. Only the
    /// value's head is looked at; its arguments, items and fields are resolved
    /// once the sort is chosen.
    fn stands_at(&self, value: &Value, sort: &SortKey) -> bool {
        match &value.kind {
            ValueKind::Parameter(_) => unreachable!("a parameter is resolved before stands_at"),
            ValueKind::Call(call) => self
                .registry
                .contains_key(&(call.head.ident.as_str().to_owned(), sort.clone())),
            ValueKind::String(_) => coercion::string_stands_at(sort),
            ValueKind::Integer(number) => coercion::integer_stands_at(*number, sort),
            ValueKind::List(_) => coercion::list_stands_at(sort),
            ValueKind::Record(_) => coercion::record_stands_at(sort),
        }
    }

    /// Resolves `value` through the coercion into `expected` whose from-sort
    /// it stands at, if there is one. A value standing at two from-sorts is
    /// refused, never resolved by declaration order. Whether `value` was
    /// resolved or refused here.
    fn coerce(&mut self, value: &Value, expected: &SortKey) -> bool {
        let coercions = self.coercions;
        let reached: Vec<&Coercion> = coercions
            .iter()
            .filter(|c| c.to == *expected && self.stands_at(value, &c.from))
            .collect();
        match reached.as_slice() {
            [] => false,
            [coercion] => {
                self.edges.push(coercion.node);
                self.value_at(value, &coercion.from, false);
                true
            }
            [..] => {
                self.refuse(
                    value.span,
                    RefusalKind::AmbiguousCoercion {
                        sort: expected.to_string(),
                        from: reached.iter().map(|c| c.from.to_string()).collect(),
                    },
                );
                true
            }
        }
    }

    /// `$name` at `expected`: the parameter's own sort, `Option` of it, or a
    /// sort it is coerced into.
    fn parameter(&mut self, name: &str, span: Span, expected: &SortKey, coerce: bool) {
        let Some(param) = self.params.iter().find(|p| p.name == name) else {
            return self.refuse(span, RefusalKind::UnknownParameter(name.to_owned()));
        };
        let sort = &param.sort;
        let at = expected.option_inner().unwrap_or(expected);
        if sort == expected || sort == at {
            return;
        }
        // Duplicate coercions are refused and dropped at load, so at most one
        // goes from the parameter's sort into `at`.
        let coercion = self
            .coercions
            .iter()
            .find(|c| c.from == *sort && c.to == *at);
        match coercion {
            Some(coercion) if coerce => self.edges.push(coercion.node),
            Some(_) | None => self.refuse(
                span,
                RefusalKind::ParameterSort {
                    name: name.to_owned(),
                    sort: sort.to_string(),
                    expected: expected.to_string(),
                },
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Every sort the kind and record tables write is one the plugins
    /// produce and consume, so the tables name no misspelled sort.
    #[test]
    fn every_table_sort_is_known() {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
        let canon = Bundle::load(plugins.join("canon"), builtin.next_file_id()).unwrap();
        let decls: Vec<Decl<'_>> = [&builtin, &canon]
            .into_iter()
            .flat_map(|bundle| bundle.files().values())
            .flat_map(|file| file.ast.declarations())
            .map(|declaration| header(declaration, false).expect("the plugins' headers pass"))
            .collect();
        let registry = register(&decls, &mut Vec::new());
        let known = known_sorts(&decls, &registry);
        let mut written = Vec::new();
        for kind in KINDS {
            let fields = kind.fields.iter().filter_map(|field| field.sort);
            written.extend(kind.sort.into_iter().chain(fields));
            written.extend(kind.secondary.map(|secondary| secondary.sort));
        }
        for record in RECORDS {
            written.push(record.sort);
            written.extend(record.fields.iter().map(|(_, sort, _)| *sort));
        }
        for sort in written {
            let mut heads = BTreeSet::new();
            let key = SortKey::table(sort);
            key.heads(&mut heads);
            for head in heads {
                assert!(
                    known.contains(head),
                    "table sort `{sort}`: `{head}` is unknown"
                );
            }
        }
    }
}
