//! `cargo xtask dsl check`: every reference in the `.syl` bundles resolves.
//!
//! A temporary validator that guards `.syl` while there is neither the Lean
//! workbench nor the Rust resolution layer (step 7 of the semantics-v3 plan),
//! which replaces it. It checks names, sorts and arity only, following
//! `docs/decisions/syl.md` §3; it expands nothing and reads no meaning.
//!
//! [`check`] first runs [`sylex_syl::check()`] (constructor clauses, and words
//! that name no declaration at any sort), then resolves every value at its
//! expected sort against the `(name, sort)` signatures the declaration headers
//! declare. Declaration kinds, record sorts and reserved
//! constructors have no `.syl` declaration; their provisional schemas are
//! [`schema`]'s. The only words and sorts written here are the language's own
//! (`none`, `some`, `Option`, `List` and the literal sorts, `syl.md` §3) and
//! those of the single-face card exception.

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
use sylex_syl::Call;
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

pub use self::refusal::CheckError;
pub use self::refusal::Refusal;
pub use self::refusal::RefusalKind;
use self::schema::Form;
use self::schema::KINDS;
use self::schema::Kind;
use self::schema::RECORDS;
use self::schema::TextWord;
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
    /// `None`: written but not resolved, for the one temporary exception that
    /// needs it (`schema::TextWord`).
    sort: Option<SortKey>,
    optional: bool,
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
            Shape::Named(named) => named.body.as_ref(),
            Shape::Nameless(_) => None,
        }
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
    known_sorts(&decls, &registry, &mut refusals);
    let mut edges = vec![Vec::new(); decls.len()];
    for (decl, edges) in decls.iter().zip(&mut edges) {
        resolve(decl, &registry, &mut refusals, edges);
    }
    for cycle in cycles::cycles(&edges) {
        let name = |i: usize| {
            decls[i]
                .name
                .clone()
                .expect("only named declarations are nodes")
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
    let named = match (&declaration.shape, kind.form) {
        (Shape::Named(named), Form::Block | Form::OptionalBlock) => named,
        (Shape::Nameless(_), Form::Value) => {
            let sort = kind.sort.expect("a nameless kind implies its sort");
            return Ok(decl(None, SortKey::table(sort), Vec::new()));
        }
        (Shape::Nameless(_), Form::Block | Form::OptionalBlock) => {
            return refuse(declaration.span, RefusalKind::MissingName(word.to_owned()));
        }
        (Shape::Named(_), Form::Value) => {
            return refuse(declaration.span, RefusalKind::WrittenName(word.to_owned()));
        }
    };
    let name = named.name.ident.as_str();
    if name == "none" {
        return refuse(named.name.span, RefusalKind::NamedNone);
    }
    let sort = match (kind.sort, &named.sort) {
        (None, Some(sort)) => sort_key(sort)?,
        (None, None) => return refuse(named.span, RefusalKind::MissingSort(word.to_owned())),
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
    if named.body.is_none() && kind.form == Form::Block {
        return refuse(named.span, RefusalKind::MissingBlock(word.to_owned()));
    }
    let mut params: Vec<Slot> = Vec::new();
    for param in named.params.iter().filter_map(Item::node) {
        let param_name = param.name.ident.as_str();
        if params.iter().any(|p| p.name == param_name) {
            return refuse(
                param.name.span,
                RefusalKind::DuplicateParameter(param_name.to_owned()),
            );
        }
        params.push(Slot {
            name: param_name.to_owned(),
            sort: Some(sort_key(&param.sort)?),
            optional: param.default.is_some(),
        });
    }
    Ok(decl(Some(name.to_owned()), sort, params))
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
        let Shape::Named(named) = &decl.declaration.shape else {
            unreachable!("a named declaration has a named shape");
        };
        let mut ring_only = false;
        if let Some(constructor) = named.constructor
            && let Some(reserved) =
                schema::reserved(&decl.sort.to_string(), constructor.ident.as_str())
        {
            ring_only = decl.ring && reserved == schema::RING_EXCEPTION.1;
            if !ring_only {
                refusals.push(Refusal {
                    span: constructor.span,
                    kind: RefusalKind::ReservedConstructor(reserved),
                });
            }
        }
        let signature = Signature {
            params: decl.params.clone(),
            node: (!decl.kind.catalog).then_some(index),
            ring_only,
        };
        let span = decl.declaration.span;
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
        };
        let key = (name.clone(), SortKey::named(secondary.sort));
        insert(key, signature, span, refusals);
    }
    registry
}

/// The sorts a literal stands at (`syl.md` §3).
const LITERAL_SORTS: [&str; 3] = ["String", "Int", "Nat"];

/// Refuses a written sort name that no declaration's sort, record sort,
/// literal sort or sort constructor names.
fn known_sorts(decls: &[Decl<'_>], registry: &Registry, refusals: &mut Vec<Refusal>) {
    let mut known: BTreeSet<&str> = LITERAL_SORTS
        .into_iter()
        .chain(schema::SORT_CONSTRUCTORS)
        .chain(RECORDS.iter().map(|record| record.sort))
        .chain(schema::UNDECLARED_SORTS.iter().copied())
        .collect();
    for (_, sort) in registry.keys() {
        sort.heads(&mut known);
    }
    for decl in decls {
        let Shape::Named(named) = &decl.declaration.shape else {
            continue;
        };
        let params = named.params.iter().filter_map(Item::node).map(|p| &p.sort);
        for sort in named.sort.iter().chain(params) {
            unknown_sort(sort, &known, refusals);
        }
    }
}

fn unknown_sort(sort: &Sort, known: &BTreeSet<&str>, refusals: &mut Vec<Refusal>) {
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
    refusals: &mut Vec<Refusal>,
    edges: &mut Vec<usize>,
) {
    let ring = decl.ring;
    if let Shape::Named(named) = &decl.declaration.shape {
        // A default reads only the parameters before it.
        for (index, param) in named.params.iter().filter_map(Item::node).enumerate() {
            if let Some(default) = &param.default {
                let sort = decl.params[index]
                    .sort
                    .as_ref()
                    .expect("parameters have sorts");
                let params = &decl.params[..index];
                Resolver {
                    registry,
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
        let (named, name) = match (&decl.declaration.shape, &decl.name) {
            (Shape::Nameless(value), None) => return self.value(value, &decl.sort),
            (Shape::Named(named), Some(name)) => (named, name),
            (Shape::Nameless(_), Some(_)) | (Shape::Named(_), None) => {
                unreachable!("a declaration has a name exactly when its shape is named")
            }
        };
        // Only a kind whose block is optional may omit it, and then there is
        // nothing to resolve.
        let Some(block) = &named.body else {
            return;
        };
        let at = format!("`{}` {name}", decl.kind.word);
        let own_fields = decl
            .kind
            .fields
            .iter()
            .any(|spec| decl.field(spec.name).is_some());
        if decl.kind.record_fields && !own_fields {
            return self.record(block, &decl.sort);
        }
        let mut slots: Vec<Slot> = decl
            .kind
            .fields
            .iter()
            .map(|spec| Slot {
                name: spec.name.to_owned(),
                sort: Some(spec.sort.map_or_else(|| decl.sort.clone(), SortKey::table)),
                optional: !spec.required,
            })
            .collect();
        // Temporary: `syl-subtype-card-types` deletes this exception.
        if let Some(TextWord { field, word }) = decl.kind.text_word
            && decl.field(field).is_some_and(|value| is_bare(value, word))
        {
            let slot = slots.iter_mut().find(|slot| slot.name == field);
            slot.expect("the text word's field is the kind's").sort = None;
        }
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
                Some(slot) => {
                    if let Some(sort) = &slot.sort {
                        self.value(&field.value, sort);
                    }
                }
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
                sort: Some(SortKey::table(sort)),
                optional: *optional,
            })
            .collect();
        self.fields(record, &slots, spec.sort);
    }

    /// Resolves `value` at `expected` (`syl.md` §3).
    fn value(&mut self, value: &Value, expected: &SortKey) {
        match &value.kind {
            ValueKind::Record(record) => {
                // Temporary: a record at `Card` is a single-faced card, and a
                // `CardFace` record without `characteristics` is its
                // characteristics: two implicit wraps.
                // `syl-single-face-card-call` deletes this exception.
                if expected.head == "Card" {
                    return self.value(value, &SortKey::named("CardFace"));
                }
                if expected.head == "CardFace"
                    && !record.nodes().any(|f| f.name.ident == "characteristics")
                {
                    return self.record(record, &SortKey::named("Characteristics"));
                }
            }
            ValueKind::Parameter(name) => {
                return self.parameter(name.ident.as_str(), value.span, expected);
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
            ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::List(_) => {}
        }
        // The one implicit wrap: a `T` at `Option(T)`.
        if let Some(inner) = expected.option_inner() {
            return self.value(value, inner);
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

    /// `$name` at `expected`: the parameter's own sort, or `Option` of it.
    fn parameter(&mut self, name: &str, span: Span, expected: &SortKey) {
        let Some(param) = self.params.iter().find(|p| p.name == name) else {
            return self.refuse(span, RefusalKind::UnknownParameter(name.to_owned()));
        };
        let sort = param.sort.as_ref().expect("parameters have sorts");
        if sort != expected && expected.option_inner() != Some(sort) {
            self.refuse(
                span,
                RefusalKind::ParameterSort {
                    name: name.to_owned(),
                    sort: sort.to_string(),
                    expected: expected.to_string(),
                },
            );
        }
    }

    /// Binds a call's arguments in written order: a named argument binds its
    /// parameter, a positional one the first parameter not yet bound.
    fn call(&mut self, call: &Call, signature: &Signature) {
        let word = call.head.ident.as_str();
        // Temporary: `syl-ring-tempts-you-raw-scope` deletes this exception.
        if signature.ring_only && !self.ring {
            let reserved = schema::RING_EXCEPTION.1.to_owned();
            self.refuse(call.head.span, RefusalKind::ReservedConstructor(reserved));
        }
        self.edges.extend(signature.node);
        let mut bound: Vec<&str> = Vec::new();
        for arg in call.args.iter().filter_map(Item::node) {
            let param = if let Some(name) = arg.name {
                signature
                    .params
                    .iter()
                    .find(|p| p.name == name.ident.as_str())
            } else {
                signature
                    .params
                    .iter()
                    .find(|p| !bound.contains(&p.name.as_str()))
            };
            let Some(param) = param else {
                let kind = match arg.name {
                    Some(name) => RefusalKind::UnknownArgument {
                        word: word.to_owned(),
                        argument: name.ident.as_str().to_owned(),
                    },
                    None => RefusalKind::TooManyArguments(word.to_owned()),
                };
                self.refuse(arg.name.map_or(arg.span, |name| name.span), kind);
                continue;
            };
            if bound.contains(&param.name.as_str()) {
                self.refuse(
                    arg.span,
                    RefusalKind::DuplicateArgument {
                        word: word.to_owned(),
                        argument: param.name.clone(),
                    },
                );
                continue;
            }
            bound.push(&param.name);
            self.value(
                &arg.value,
                param.sort.as_ref().expect("parameters have sorts"),
            );
        }
        for param in &signature.params {
            if !param.optional && !bound.contains(&param.name.as_str()) {
                self.refuse(
                    call.span,
                    RefusalKind::MissingArgument {
                        word: word.to_owned(),
                        argument: param.name.clone(),
                    },
                );
            }
        }
    }
}
