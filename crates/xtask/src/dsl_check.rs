//! `cargo xtask dsl check`: every reference in the `.syl` bundles resolves.
//!
//! A temporary validator that guards `.syl` until the Rust resolution layer
//! replaces it. It checks names, sorts, argument binding and each declaration's
//! `spelling` notation ([`sylex_spelling`]), following `docs/design/syl.md` §2
//! and §3 and `docs/design/spelling.md` §7; it expands nothing and reads no
//! meaning.
//!
//! [`check`] first runs [`sylex_syl::check()`] (constructor clauses, and words
//! that name no declaration at any sort), then resolves every value at its
//! expected sort against the `(name, sort)` signatures the declaration headers
//! declare. Declaration kinds, record sorts and reserved
//! constructors have no `.syl` declaration; their provisional schemas are
//! [`schema`]'s. The only words and sorts written here are the language's own
//! (`none`, `some`, `Option`, `List` and the literal sorts, `syl.md` §3).
//! The memberships between sorts are declared in `.syl`; [`membership`] reads
//! and checks them. A card's term is resolved in surface mode and every other
//! value in body mode (`syl.md` §3).
//!
//! The resolver records what it resolved ([`Resolutions`]) for
//! [`crate::dsl_expand`], the throwaway expander; [`load`] returns the record
//! with the registry it was resolved against.

mod call;
mod cycles;
mod membership;
mod nominal;
mod refusal;
pub(crate) mod schema;
mod self_name;
mod sort;
mod spelling;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::btree_map::Entry;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Args;
use sylex_syl::Arg;
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

use self::call::WrittenForms;
pub(crate) use self::membership::Mode;
pub(crate) use self::membership::Sorts;
pub use self::refusal::CheckError;
pub use self::refusal::Refusal;
pub use self::refusal::RefusalKind;
use self::schema::Form;
use self::schema::KINDS;
pub(crate) use self::schema::Kind;
use self::schema::RECORDS;
pub(crate) use self::sort::SortKey;
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
    /// The `(card file name, test)` exceptions the cards' terms used
    /// (`schema::TEST_EXCEPTIONS`).
    pub test_exceptions: BTreeSet<(String, String)>,
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
        write!(
            f,
            "); {} test exceptions until {}",
            self.test_exceptions.len(),
            schema::TEST_EXCEPTION_TICKET
        )
    }
}

/// A parameter of a declaration, or a field of a record or block.
#[derive(Debug, Clone)]
pub(crate) struct Slot {
    pub(crate) name: String,
    pub(crate) sort: SortKey,
    optional: bool,
    /// A parameter's declared default; `None` for a field.
    default: Option<Value>,
}

/// What a word resolves to at one sort.
#[derive(Debug)]
pub(crate) struct Signature {
    params: Vec<Slot>,
    /// The declaration's index in [`Checked::decls`].
    pub(crate) decl: usize,
    /// Registered at its kind's second sort (`syl.md` §3), which names the
    /// declaration and calls nothing.
    pub(crate) secondary: bool,
    /// The declaration's node in the dependency graph; `None` for a second
    /// registration and a catalog declaration, which are no dependency edge.
    node: Option<usize>,
    /// One sort of a declaration with an `at` list: a call of the word stands
    /// at the lowest of its sorts that holds its arms (`syl.md` §3).
    pub(crate) at_list: bool,
    /// Declared under the Ring exception (`schema::RING_EXCEPTION`), so only
    /// the Ring's file may call it.
    ring_only: bool,
    /// The declaration's span.
    span: Span,
}

pub(crate) type Registry = BTreeMap<(String, SortKey), Signature>;

/// What the resolver resolved, keyed by the address of the syntax node in its
/// bundle, for [`crate::dsl_expand`].
#[derive(Debug, Default)]
pub(crate) struct Resolutions {
    /// The sort each call's word was found at, its registry key's sort, for
    /// each sort of the enclosing declaration's `at` list (`None` when it has
    /// none).
    calls: HashMap<(*const Call, Option<SortKey>), SortKey>,
    /// The index of the parameter each argument binds.
    args: HashMap<*const Arg, usize>,
}

impl Resolutions {
    /// The sort `call`'s word was found at, if it was resolved. Inside a
    /// declaration with an `at` list, `self_sort` is the sort of that list
    /// the declaration is read at.
    pub(crate) fn call(&self, call: &Call, self_sort: Option<&SortKey>) -> Option<SortKey> {
        self.calls
            .get(&(std::ptr::from_ref(call), self_sort.cloned()))
            .cloned()
    }

    /// The index of the parameter `arg` binds, if it was bound.
    pub(crate) fn arg(&self, arg: &Arg) -> Option<usize> {
        self.args.get(&std::ptr::from_ref(arg)).copied()
    }
}

/// Everything a passing check resolved.
pub(crate) struct Checked<'a> {
    pub(crate) summary: Summary,
    pub(crate) decls: Vec<Decl<'a>>,
    pub(crate) registry: Registry,
    pub(crate) resolutions: Resolutions,
}

/// One declaration whose header passed.
pub(crate) struct Decl<'a> {
    pub(crate) declaration: &'a Declaration,
    pub(crate) kind: &'static Kind,
    /// `None` for a kind written without a name, which declares no word.
    pub(crate) name: Option<String>,
    /// The sort it is declared at: `Self` when it has an `at` list.
    pub(crate) sort: SortKey,
    pub(crate) params: Vec<Slot>,
    /// The sorts of its `at` list; empty when it has none.
    pub(crate) at: Vec<SortKey>,
    /// A `member` or `denotes` declaration's subject sort.
    pub(crate) subject: Option<SortKey>,
    /// A card's file name, without `.syl`.
    pub(crate) card: Option<String>,
    /// Whether this declaration stands in the builtin bundle.
    pub(crate) in_builtin: bool,
    /// Whether this is the Ring exception's file.
    ring: bool,
}

impl<'a> Decl<'a> {
    pub(crate) fn block(&self) -> Option<&'a Record> {
        match &self.declaration.shape {
            Shape::Header(header) => header.body.as_ref(),
            Shape::Value(_) | Shape::Relation(_) => None,
        }
    }

    /// The sorts the declaration is checked at: each sort of its `at` list,
    /// or its one sort.
    pub(crate) fn instances(&self) -> Vec<Option<&SortKey>> {
        if self.at.is_empty() { vec![None] } else { self.at.iter().map(Some).collect() }
    }

    /// The declaration as a refusal names it: its kind word and its name, or
    /// for a relation its two sorts.
    pub(crate) fn label(&self) -> String {
        let word = self.kind.word;
        if let Some(name) = &self.name {
            return format!("`{word}` {name}");
        }
        match &self.subject {
            Some(subject) => format!("`{word}`({subject}): {}", self.sort),
            None => format!("`{word}`"),
        }
    }

    pub(crate) fn field(&self, name: &str) -> Option<&'a Value> {
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
    load(&bundles).map(|checked| checked.summary)
}

/// Checks `bundles`, the builtin bundle first, and returns what was resolved.
pub(crate) fn load<'a>(bundles: &[&'a Bundle]) -> Result<Checked<'a>, CheckError> {
    let builtin = bundles[0];
    sylex_syl::check(bundles).map_err(CheckError::Declarations)?;
    let ring_file = builtin.root().join(schema::RING_EXCEPTION.0);
    let mut refusals = Vec::new();
    let mut decls = Vec::new();
    let mut summary = Summary {
        files: 0,
        declarations: BTreeMap::new(),
        test_exceptions: BTreeSet::new(),
    };
    for bundle in bundles {
        let in_builtin = std::ptr::eq(*bundle, builtin);
        for file in bundle.files().values() {
            summary.files += 1;
            let ring = in_builtin && file.path == ring_file;
            let card = file
                .path
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned());
            for declaration in file.ast.declarations() {
                match header(declaration, in_builtin, ring) {
                    Ok(mut decl) => {
                        if decl.kind.word == schema::CARD {
                            decl.card.clone_from(&card);
                        }
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
    let sorts = Sorts::read(&decls, &mut refusals);
    membership::ambiguities(&sorts, &registry, &mut refusals);
    placement(&decls, &sorts, &mut refusals);
    let mut words: BTreeMap<&str, Vec<&SortKey>> = BTreeMap::new();
    for (name, sort) in registry.keys() {
        words.entry(name.as_str()).or_default().push(sort);
    }
    let forms = call::written_forms(&decls, &registry, &sorts);
    let shared_verbs = shared_verb_lemmas(&decls, &mut refusals);
    let tables = Tables {
        decls: &decls,
        registry: &registry,
        words: &words,
        sorts: &sorts,
        forms: &forms,
        shared_verbs: &shared_verbs,
    };
    let mut edges = vec![Vec::new(); decls.len()];
    let mut resolutions = Resolutions::default();
    let mut exceptions_used = BTreeSet::new();
    for (decl, edges) in decls.iter().zip(&mut edges) {
        resolve(
            decl,
            &tables,
            &mut refusals,
            edges,
            &mut resolutions,
            &mut exceptions_used,
        );
    }
    let found_cycles = cycles::cycles(&edges);
    for cycle in found_cycles {
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
        nominal::check(&decls, &registry, &resolutions, &mut refusals);
    }
    if refusals.is_empty() {
        self_name::check(&decls, &registry, &resolutions, &mut refusals);
    }
    // A declaration with an `at` list is resolved once for each of its sorts,
    // so one defect in it is found once for each.
    let mut seen = Vec::new();
    refusals.retain(|refusal| {
        let new = !seen.contains(refusal);
        if new {
            seen.push(refusal.clone());
        }
        new
    });
    summary.test_exceptions = exceptions_used;
    if refusals.is_empty() {
        return Ok(Checked {
            summary,
            decls,
            registry,
            resolutions,
        });
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

/// Checks a declaration's kind, name, sort, parameters and `at` list.
fn header(declaration: &Declaration, in_builtin: bool, ring: bool) -> Result<Decl<'_>, Refusal> {
    let refuse = |span, kind| Err(Refusal { span, kind });
    let word = declaration.kind.ident.as_str();
    let Some(kind) = KINDS.iter().find(|kind| kind.word == word) else {
        return refuse(
            declaration.kind.span,
            RefusalKind::UnknownKind(word.to_owned()),
        );
    };
    let decl = |name, sort, params, at, subject| Decl {
        declaration,
        kind,
        name,
        sort,
        params,
        at,
        subject,
        card: None,
        in_builtin,
        ring,
    };
    let header = match (&declaration.shape, kind.form) {
        (Shape::Value(_), Form::Value) => {
            let sort = kind.sort.expect("a value kind implies its sort");
            return Ok(decl(
                None,
                SortKey::table(sort),
                Vec::new(),
                Vec::new(),
                None,
            ));
        }
        (Shape::Relation(relation), Form::Relation) => {
            let subject = sort_key(&relation.subject)?;
            let sort = sort_key(&relation.sort)?;
            return Ok(decl(None, sort, Vec::new(), Vec::new(), Some(subject)));
        }
        (Shape::Header(header), Form::Block | Form::OptionalBlock) => header,
        (Shape::Value(_) | Shape::Relation(_), Form::Block | Form::OptionalBlock) => {
            return refuse(declaration.span, RefusalKind::MissingName(word.to_owned()));
        }
        (Shape::Value(_) | Shape::Header(_), Form::Relation) => {
            return refuse(declaration.span, RefusalKind::RelationForm(word.to_owned()));
        }
        (Shape::Header(_), Form::Value) => {
            return refuse(declaration.span, RefusalKind::WrittenName(word.to_owned()));
        }
        (Shape::Relation(_), Form::Value) => {
            return refuse(
                declaration.span,
                RefusalKind::WrittenHeader(word.to_owned()),
            );
        }
    };
    if header.name.ident == "none" {
        return refuse(header.name.span, RefusalKind::NamedNone);
    }
    let name = header.name.ident.as_str();
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
    if header.body.is_none() && kind.form == Form::Block {
        return refuse(header.span, RefusalKind::MissingBlock(word.to_owned()));
    }
    let mut params: Vec<Slot> = Vec::new();
    for param in header.params.iter().filter_map(Item::node) {
        let param_name = param.name.ident.as_str();
        if params.iter().any(|p| p.name == param_name)
            || kind.implicit.iter().any(|(name, _)| *name == param_name)
        {
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
    let mut at: Vec<SortKey> = Vec::new();
    for listed in header.at_sorts() {
        if kind.sort.is_some() {
            return refuse(listed.span, RefusalKind::AtListOnKind(word.to_owned()));
        }
        let listed_key = sort_key(listed)?;
        if listed_key.mentions_self() {
            return refuse(listed.span, RefusalKind::SelfWithoutAtList);
        }
        if at.contains(&listed_key) {
            return refuse(
                listed.span,
                RefusalKind::DuplicateAtSort(listed_key.to_string()),
            );
        }
        at.push(listed_key);
    }
    let mentions_self = sort.mentions_self() || params.iter().any(|p| p.sort.mentions_self());
    if at.is_empty() && mentions_self {
        return refuse(header.span, RefusalKind::SelfWithoutAtList);
    }
    if !at.is_empty() && sort != SortKey::self_sort() {
        return refuse(header.span, RefusalKind::AtListWithoutSelf);
    }
    Ok(decl(Some(name.to_owned()), sort, params, at, None))
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
        for instance in decl.instances() {
            let instantiate = |sort: &SortKey| match instance {
                Some(instance) => sort.instantiate(instance),
                None => sort.clone(),
            };
            let signature = Signature {
                params: decl
                    .params
                    .iter()
                    .map(|param| Slot {
                        sort: instantiate(&param.sort),
                        ..param.clone()
                    })
                    .collect(),
                decl: index,
                secondary: false,
                node: (!decl.kind.catalog).then_some(index),
                at_list: instance.is_some(),
                ring_only,
                span,
            };
            insert(
                (name.clone(), instantiate(&decl.sort)),
                signature,
                span,
                refusals,
            );
        }
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
            decl: index,
            secondary: true,
            node: None,
            at_list: false,
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
/// consumed. A declaration's result sort, a record sort or a membership's
/// subject produces a name; a parameter's sort, a kind's sort or field sort, a
/// record field's sort or a membership's sort consumes one. A sort that a kind
/// denotes is consumed where that kind is and produced where that kind is, and
/// a kind that denotes one likewise. A name only produced or only consumed is a
/// misspelling, or a sort no value can reach.
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
    produced.extend(RECORDS.iter().map(|record| record.sort.to_owned()));
    // A literal is a value of its sort, so a literal sort is produced, and a
    // sort it is a member of with it.
    produced.extend(LITERAL_SORTS.map(str::to_owned));
    let mut consumed = BTreeSet::new();
    for decl in decls {
        for param in &decl.params {
            for instance in decl.instances() {
                match instance {
                    Some(instance) => add(&param.sort.instantiate(instance), &mut consumed),
                    None => add(&param.sort, &mut consumed),
                }
            }
        }
    }
    // A member's values are values of the sort it is a member of, so what
    // produces the one produces the other and what consumes the other
    // consumes the one. `denotes` joins its two sorts both ways.
    let relations: Vec<(&SortKey, &SortKey, bool)> = decls
        .iter()
        .filter_map(|decl| {
            let subject = decl.subject.as_ref()?;
            Some((subject, &decl.sort, decl.kind.word == schema::DENOTES))
        })
        .collect();
    loop {
        let before = (produced.len(), consumed.len());
        for (subject, sort, both_ways) in &relations {
            let (subject, sort) = (subject.head.as_str(), sort.head.as_str());
            if produced.contains(subject) {
                produced.insert(sort.to_owned());
            }
            if consumed.contains(sort) {
                consumed.insert(subject.to_owned());
            }
            if *both_ways {
                if produced.contains(sort) {
                    produced.insert(subject.to_owned());
                }
                if consumed.contains(subject) {
                    consumed.insert(sort.to_owned());
                }
            }
        }
        if before == (produced.len(), consumed.len()) {
            break;
        }
    }
    for kind in KINDS {
        let secondary = kind.secondary.map(|secondary| secondary.sort);
        let implicit = kind.implicit.iter().map(|(_, sort)| *sort);
        let fields = kind.fields.iter().filter_map(|field| field.sort);
        for sort in kind
            .sort
            .into_iter()
            .chain(secondary)
            .chain(implicit)
            .chain(fields)
        {
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

/// Refuses every written sort name `known` lacks. `Self` is known in a
/// declaration with an `at` list.
fn unknown_sorts(decls: &[Decl<'_>], known: &BTreeSet<String>, refusals: &mut Vec<Refusal>) {
    for decl in decls {
        let self_known = !decl.at.is_empty();
        match &decl.declaration.shape {
            Shape::Value(_) => {}
            Shape::Relation(relation) => {
                unknown_sort(&relation.subject, known, false, refusals);
                unknown_sort(&relation.sort, known, false, refusals);
            }
            Shape::Header(header) => {
                let params = header.params.iter().filter_map(Item::node).map(|p| &p.sort);
                for sort in header.sort.iter().chain(params) {
                    unknown_sort(sort, known, self_known, refusals);
                }
                for sort in header.at_sorts() {
                    unknown_sort(sort, known, false, refusals);
                }
            }
        }
    }
}

fn unknown_sort(
    sort: &Sort,
    known: &BTreeSet<String>,
    self_known: bool,
    refusals: &mut Vec<Refusal>,
) {
    let head = sort.head.ident.as_str();
    let is_self = self_known && head == sylex_syl::SELF_SORT;
    if !known.contains(head) && !is_self {
        refusals.push(Refusal {
            span: sort.head.span,
            kind: RefusalKind::UnknownSort(head.to_owned()),
        });
    }
    for arg in &sort.args {
        unknown_sort(arg, known, self_known, refusals);
    }
}

/// Refuses a declaration at a position sort that is neither that sort's own
/// (`schema::POSITION_OWN`) nor one staying there for a ticket
/// (`schema::POSITION_STAYING`), and a `spelling` at a test sort, where
/// nothing prints (`syl.md` §3). A declaration whose `at` list holds another
/// sort as well stands at all its sorts and is neither.
fn placement(decls: &[Decl<'_>], sorts: &Sorts, refusals: &mut Vec<Refusal>) {
    for decl in decls {
        let Some(name) = &decl.name else {
            continue;
        };
        // The expander's probe Template stands wherever a term is asked for.
        if !decl.in_builtin && name == crate::dsl_expand::TERM {
            continue;
        }
        // A declaration with an `at` list is at a position sort, or at a test
        // sort, only when every sort of its list is one.
        let at_only = |is: &dyn Fn(&SortKey) -> bool| match decl.at.as_slice() {
            [] => is(&decl.sort),
            listed => listed.iter().all(is),
        };
        let sort = match decl.at.as_slice() {
            [] => decl.sort.to_string(),
            listed => listed
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
        };
        if at_only(&|sort| sorts.is_position(sort)) {
            let own = schema::POSITION_OWN
                .iter()
                .any(|(at, word)| *at == sort && word == name);
            let staying = schema::POSITION_STAYING
                .iter()
                .any(|(at, word, _)| *at == sort && word == name);
            if !own && !staying {
                refusals.push(Refusal {
                    span: decl.declaration.span,
                    kind: RefusalKind::PositionDeclaration {
                        word: name.clone(),
                        sort,
                    },
                });
            }
        } else if at_only(&|sort| sorts.is_test(sort))
            && let Some(spelling) = decl.field("spelling")
        {
            refusals.push(Refusal {
                span: spelling.span,
                kind: RefusalKind::SpellingAtTest {
                    word: name.clone(),
                    sort,
                },
            });
        }
    }
}

fn is_bare(value: &Value, word: &str) -> bool {
    matches!(&value.kind, ValueKind::Call(call) if call.is_bare() && call.head.ident == word)
}

/// Returns the non-empty `lemma` string and its span when `record` defines a
/// non-empty string `lemma` and a non-empty `frames` list.
fn verb_record_lemma(record: &Record) -> Option<(&str, Span)> {
    let mut written = BTreeSet::new();
    for field in record.nodes() {
        if !written.insert(field.name.ident.as_str()) {
            return None;
        }
    }
    let lemma = record.nodes().find(|f| f.name.ident == "lemma")?;
    let ValueKind::String(text) = &lemma.value.kind else {
        return None;
    };
    if text.is_empty() {
        return None;
    }
    let frames = record.nodes().find(|f| f.name.ident == "frames")?;
    let ValueKind::List(items) = &frames.value.kind else {
        return None;
    };
    if !items.iter().any(|i| i.node().is_some()) {
        return None;
    }
    Some((text.as_str(), lemma.value.span))
}

/// Collects the lemmas of `verb` declarations (lowercase lemma -> declaration
/// name), refusing a `verb` declaration without a `lexical` record defining a
/// non-empty `lemma` and `frames`, or two `verb` declarations for the same
/// lemma.
fn shared_verb_lemmas(decls: &[Decl<'_>], refusals: &mut Vec<Refusal>) -> BTreeMap<String, String> {
    let mut lemmas: BTreeMap<String, String> = BTreeMap::new();
    for decl in decls.iter().filter(|d| d.kind.word == "verb") {
        let name = decl.name.as_deref().expect("a verb declaration has a name");
        let Some(lexical) = decl.field("lexical") else {
            refusals.push(Refusal {
                span: decl.declaration.span,
                kind: RefusalKind::InvalidSharedVerb(name.to_owned()),
            });
            continue;
        };
        let ValueKind::Record(record) = &lexical.kind else {
            // `Resolver::lexical` refuses non-record `lexical` across all
            // declarations.
            continue;
        };
        let Some((lemma, span)) = verb_record_lemma(record) else {
            refusals.push(Refusal {
                span: lexical.span,
                kind: RefusalKind::InvalidSharedVerb(name.to_owned()),
            });
            continue;
        };
        let key = lemma.to_lowercase();
        if let Some(existing) = lemmas.get(&key) {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::DuplicateSharedVerb {
                    lemma: lemma.to_owned(),
                    verb: existing.clone(),
                },
            });
        } else {
            lemmas.insert(key, name.to_owned());
        }
    }
    lemmas
}

struct Tables<'r> {
    decls: &'r [Decl<'r>],
    registry: &'r Registry,
    /// The sorts each word is declared at.
    words: &'r BTreeMap<&'r str, Vec<&'r SortKey>>,
    sorts: &'r Sorts,
    forms: &'r WrittenForms,
    shared_verbs: &'r BTreeMap<String, String>,
}

/// Resolves one declaration's parameter defaults and block or value, once
/// for each sort of its `at` list.
fn resolve(
    decl: &Decl<'_>,
    tables: &Tables<'_>,
    refusals: &mut Vec<Refusal>,
    edges: &mut Vec<usize>,
    record: &mut Resolutions,
    exceptions_used: &mut BTreeSet<(String, String)>,
) {
    let ring = decl.ring;
    let mode = if decl.kind.word == schema::CARD { Mode::Surface } else { Mode::Body };
    let exceptions: Vec<&'static str> = decl.card.as_deref().map_or_else(Vec::new, |card| {
        schema::TEST_EXCEPTIONS
            .iter()
            .filter(|(file, _)| *file == card)
            .map(|(_, word)| *word)
            .collect()
    });
    for instance in decl.instances() {
        let instantiate = |sort: &SortKey| match instance {
            Some(instance) => sort.instantiate(instance),
            None => sort.clone(),
        };
        let params: Vec<Slot> = decl
            .params
            .iter()
            .map(|param| Slot {
                sort: instantiate(&param.sort),
                ..param.clone()
            })
            .collect();
        macro_rules! resolver {
            ($params:expr, $ability_template_body:expr) => {
                Resolver {
                    decls: tables.decls,
                    registry: tables.registry,
                    words: tables.words,
                    sorts: tables.sorts,
                    forms: tables.forms,
                    shared_verbs: tables.shared_verbs,
                    refusals: &mut *refusals,
                    edges: &mut *edges,
                    record: &mut *record,
                    exceptions_used: &mut *exceptions_used,
                    params: $params.to_vec(),
                    ability_template_body: $ability_template_body,
                    ring,
                    mode,
                    self_sort: instance.cloned(),
                    card: decl.card.as_deref(),
                    exceptions: &exceptions,
                }
            };
        }
        if let Shape::Header(header) = &decl.declaration.shape {
            // A default reads only the parameters before it.
            for (index, param) in header.params.iter().filter_map(Item::node).enumerate() {
                if let Some(default) = &param.default {
                    resolver!(&params[..index], None).value(default, &params[index].sort);
                }
            }
        }
        let block_params: Vec<Slot> = match &decl.declaration.shape {
            Shape::Header(_) => params
                .iter()
                .cloned()
                .chain(decl.kind.implicit.iter().map(|(name, sort)| Slot {
                    name: (*name).to_owned(),
                    sort: SortKey::table(sort),
                    optional: false,
                    default: None,
                }))
                .collect(),
            Shape::Value(_) | Shape::Relation(_) => Vec::new(),
        };
        let ability_template_body = match (
            decl.kind.word,
            decl.name.as_deref(),
            decl.field("body").map(|v| &v.kind),
        ) {
            (schema::TEMPLATE, Some(name), Some(ValueKind::Call(body_call)))
                if decl.sort == SortKey::named(schema::TRIGGERED_ABILITY.0) =>
            {
                Some((name, body_call))
            }
            _ => None,
        };
        resolver!(&block_params, ability_template_body).body(decl);
    }
}

/// Where a `{ ... }` field list stands.
#[derive(Clone, Copy)]
enum FieldHost<'a, 'd> {
    /// A declaration's `{ ... }` block, which may carry `lexical` and
    /// `spelling`.
    DeclarationBlock(&'d Decl<'a>),
    /// A record value `{ ... }`, which carries only its record sort's fields.
    RecordValue,
}

/// Why a call of a word with an `at` list, or one of its arms, has no sort.
type NoSort = (Span, RefusalKind);

/// Resolves values for one declaration.
struct Resolver<'r> {
    decls: &'r [Decl<'r>],
    registry: &'r Registry,
    words: &'r BTreeMap<&'r str, Vec<&'r SortKey>>,
    sorts: &'r Sorts,
    forms: &'r WrittenForms,
    shared_verbs: &'r BTreeMap<String, String>,
    refusals: &'r mut Vec<Refusal>,
    /// The dependency nodes the declaration reaches.
    edges: &'r mut Vec<usize>,
    record: &'r mut Resolutions,
    exceptions_used: &'r mut BTreeSet<(String, String)>,
    /// The parameters a `$name` may read.
    params: Vec<Slot>,
    /// When resolving `template <name>: Ability`, `<name>` and its root `body`
    /// call so `when`, `whenever` and `at` may expand to
    /// `triggered(<name>(…))`.
    ability_template_body: Option<(&'r str, &'r Call)>,
    ring: bool,
    mode: Mode,
    /// The sort of the declaration's `at` list it is being resolved at.
    self_sort: Option<SortKey>,
    /// The card's file name, when a card's term is being resolved.
    card: Option<&'r str>,
    /// The tests this card's term may call (`schema::TEST_EXCEPTIONS`).
    exceptions: &'r [&'static str],
}

impl Resolver<'_> {
    fn refuse(&mut self, span: Span, kind: RefusalKind) {
        self.refusals.push(Refusal { span, kind });
    }

    /// `sort` with `Self` replaced by the sort the declaration is resolved
    /// at.
    fn instantiate(&self, sort: &SortKey) -> SortKey {
        match &self.self_sort {
            Some(self_sort) => sort.instantiate(self_sort),
            None => sort.clone(),
        }
    }

    fn body(&mut self, decl: &Decl<'_>) {
        let header = match &decl.declaration.shape {
            Shape::Value(value) => return self.value(value, &decl.sort),
            Shape::Relation(_) => return,
            Shape::Header(header) => header,
        };
        // Only a kind whose block is optional may omit it, and then there is
        // nothing to resolve.
        let Some(block) = &header.body else {
            return;
        };
        let at = decl.label();
        let own = self.instantiate(&decl.sort);
        let slots: Vec<Slot> = decl
            .kind
            .fields
            .iter()
            .map(|spec| Slot {
                name: spec.name.to_owned(),
                sort: spec.sort.map_or_else(|| own.clone(), SortKey::table),
                optional: !spec.required,
                default: None,
            })
            .collect();
        self.fields(block, &slots, &at, FieldHost::DeclarationBlock(decl));
    }

    /// Resolves a record's or block's fields against `slots`. On a
    /// declaration's block, `lexical` and `spelling` are checked as metadata.
    fn fields(&mut self, record: &Record, slots: &[Slot], at: &str, host: FieldHost<'_, '_>) {
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
            if let FieldHost::DeclarationBlock(decl) = host
                && METADATA_FIELDS.contains(&name)
            {
                match name {
                    // Metadata is one declaration's, whatever its `at` list.
                    "spelling" => spelling::check(decl, &field.value, self.sorts, self.refusals),
                    "lexical" => self.lexical(&field.value),
                    other => unreachable!("unhandled metadata field `{other}`"),
                }
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

    /// Checks `lexical` metadata: requires a record, refuses duplicate fields,
    /// and when `verbs` is present, checks shared `verb` references and inline
    /// verb entries (`lemma`, `frames`, and shared-verb disjointness).
    fn lexical(&mut self, value: &Value) {
        let ValueKind::Record(record) = &value.kind else {
            self.refuse(value.span, RefusalKind::LexicalNotRecord);
            return;
        };
        let mut written = BTreeSet::new();
        for field in record.nodes() {
            let name = field.name.ident.as_str();
            if !written.insert(name) {
                self.refuse(
                    field.name.span,
                    RefusalKind::DuplicateField(name.to_owned()),
                );
            }
        }
        let Some(verbs) = record
            .nodes()
            .find(|f| f.name.ident == "verbs")
            .map(|f| &f.value)
        else {
            return;
        };
        let ValueKind::List(items) = &verbs.kind else {
            self.refuse(verbs.span, RefusalKind::LexicalVerbsNotList);
            return;
        };
        let verb_sort = SortKey::named("Verb");
        for item in items.iter().filter_map(Item::node) {
            match &item.kind {
                ValueKind::Record(entry) => match verb_record_lemma(entry) {
                    None => self.refuse(item.span, RefusalKind::InvalidLexicalVerb),
                    Some((lemma, span)) => {
                        if let Some(verb) = self.shared_verbs.get(&lemma.to_lowercase()) {
                            self.refuse(
                                span,
                                RefusalKind::DuplicateSharedVerb {
                                    lemma: lemma.to_owned(),
                                    verb: verb.clone(),
                                },
                            );
                        }
                    }
                },
                ValueKind::Call(_)
                | ValueKind::String(_)
                | ValueKind::Integer(_)
                | ValueKind::List(_)
                | ValueKind::Parameter(_) => self.value(item, &verb_sort),
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
        self.fields(record, &slots, spec.sort, FieldHost::RecordValue);
    }

    /// Resolves `value` at `expected` (`syl.md` §3).
    fn value(&mut self, value: &Value, expected: &SortKey) {
        match &value.kind {
            ValueKind::Parameter(name) => self.parameter(value, name.ident.as_str(), expected),
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
                self.word(call, expected);
            }
            ValueKind::String(_)
            | ValueKind::Integer(_)
            | ValueKind::List(_)
            | ValueKind::Record(_) => self.literal(value, expected),
        }
    }

    /// The sorts of `reaches` that `word` is declared at.
    fn standing<'s>(&self, word: &str, reaches: &'s [SortKey]) -> Vec<&'s SortKey> {
        let declared = self.words.get(word).map_or(&[][..], Vec::as_slice);
        reaches
            .iter()
            .filter(|sort| declared.contains(sort))
            .collect()
    }

    fn is_at_list(&self, word: &str, sort: &SortKey) -> bool {
        self.registry
            .get(&(word.to_owned(), sort.clone()))
            .is_some_and(|signature| signature.at_list)
    }

    /// Records that `call`'s word was found at `sort`, for the sort of the
    /// `at` list the declaration is resolved at.
    fn found(&mut self, call: &Call, sort: &SortKey) {
        self.record.calls.insert(
            (std::ptr::from_ref(call), self.self_sort.clone()),
            sort.clone(),
        );
    }

    /// Resolves the call `call` at `expected`: at `expected` itself, at the
    /// sort `expected` is `Option` of, or at the one sort of the mode's
    /// Reaches(`expected`) its word is declared at. A word with an `at` list
    /// stands at the lowest of its sorts that holds its arms.
    fn word(&mut self, call: &Call, expected: &SortKey) {
        let head = call.head.ident.as_str();
        if let Some(signature) = self.registry.get(&(head.to_owned(), expected.clone()))
            && !signature.at_list
        {
            self.found(call, expected);
            return self.call(call, expected, signature);
        }
        if let Some(inner) = expected.option_inner() {
            return self.word(call, inner);
        }
        if self.mode == Mode::Surface
            && self.sorts.is_position(expected)
            && let Some(test) = self.excepted(call, expected)
        {
            let card = self.card.expect("only a card's term has exceptions");
            self.exceptions_used
                .insert((card.to_owned(), test.to_owned()));
            self.mode = Mode::Body;
            self.word(call, expected);
            self.mode = Mode::Surface;
            return;
        }
        let reaches = self.sorts.reaches(expected, self.mode);
        let standing = self.standing(head, &reaches);
        if standing.iter().any(|sort| self.is_at_list(head, sort)) {
            return match self.at_list_sort(call, expected, &reaches) {
                Ok(sort) => {
                    let signature = &self.registry[&(head.to_owned(), sort.clone())];
                    self.found(call, &sort);
                    self.call(call, &sort, signature);
                }
                Err((span, kind)) => self.refuse(span, kind),
            };
        }
        match standing.as_slice() {
            [] => {
                let kind = self.unknown_word(head, expected);
                self.refuse(call.head.span, kind);
            }
            [sort] => {
                let signature = &self.registry[&(head.to_owned(), (*sort).clone())];
                self.found(call, sort);
                self.call(call, sort, signature);
            }
            [..] => self.refuse(
                call.head.span,
                RefusalKind::AmbiguousWord {
                    word: head.to_owned(),
                    sorts: standing.iter().map(ToString::to_string).collect(),
                    target: expected.to_string(),
                    mode: self.mode.name(),
                },
            ),
        }
    }

    /// Why `word` stands nowhere in Reaches(`expected`): in a card's term, a
    /// test a body could write there; else a word unknown at `expected`.
    fn unknown_word(&self, word: &str, expected: &SortKey) -> RefusalKind {
        if self.mode == Mode::Surface {
            let body = self.sorts.reaches(expected, Mode::Body);
            if let Some(test) = self
                .standing(word, &body)
                .into_iter()
                .find(|sort| self.sorts.is_test(sort))
            {
                return RefusalKind::TestInCard {
                    word: word.to_owned(),
                    test: test.to_string(),
                };
            }
        }
        RefusalKind::UnknownWord {
            word: word.to_owned(),
            sort: expected.to_string(),
        }
    }

    /// The test of this card's exceptions that `call`, an argument of a slot
    /// typed by the position sort `expected`, calls where surface mode has no
    /// word: as its own word, or as an arm of a word with an `at` list.
    fn excepted(&self, call: &Call, expected: &SortKey) -> Option<&'static str> {
        if self.exceptions.is_empty() {
            return None;
        }
        let head = call.head.ident.as_str();
        let surface = self.sorts.reaches(expected, Mode::Surface);
        let body = self.sorts.reaches(expected, Mode::Body);
        let standing = self.standing(head, &body);
        if standing.iter().any(|sort| self.is_at_list(head, sort)) {
            return call
                .args
                .iter()
                .filter_map(Item::node)
                .flat_map(|arg| match &arg.value.kind {
                    ValueKind::List(items) => items.iter().filter_map(Item::node).collect(),
                    ValueKind::Call(_)
                    | ValueKind::Parameter(_)
                    | ValueKind::String(_)
                    | ValueKind::Integer(_)
                    | ValueKind::Record(_) => vec![&arg.value],
                })
                .find_map(|arm| match &arm.kind {
                    ValueKind::Call(arm) => self.excepted(arm, expected),
                    ValueKind::List(_)
                    | ValueKind::Parameter(_)
                    | ValueKind::String(_)
                    | ValueKind::Integer(_)
                    | ValueKind::Record(_) => None,
                });
        }
        let test = self.exceptions.iter().find(|word| **word == head)?;
        let is_test = standing.iter().any(|sort| self.sorts.is_test(sort));
        (is_test && self.standing(head, &surface).is_empty()).then_some(*test)
    }

    /// The arms of a call of a word with an `at` list: the arguments of its
    /// parameters typed by `Self`, and each item of a list written for a
    /// parameter typed `List(Self)`. Arguments are bound as [`Resolver::call`]
    /// binds them, which refuses the ones that bind nothing.
    fn arms<'c>(call: &'c Call, params: &[Slot]) -> Vec<&'c Value> {
        let mut bound = vec![false; params.len()];
        let mut arms = Vec::new();
        for arg in call.args.iter().filter_map(Item::node) {
            let index = match arg.name {
                Some(name) => params.iter().position(|p| p.name == name.ident.as_str()),
                None => (0..params.len()).find(|&i| !params[i].optional && !bound[i]),
            };
            let Some(index) = index.filter(|&i| !bound[i]) else {
                continue;
            };
            bound[index] = true;
            if !params[index].sort.mentions_self() {
                continue;
            }
            match &arg.value.kind {
                ValueKind::List(items) => arms.extend(items.iter().filter_map(Item::node)),
                ValueKind::Call(_)
                | ValueKind::Parameter(_)
                | ValueKind::String(_)
                | ValueKind::Integer(_)
                | ValueKind::Record(_) => arms.push(&arg.value),
            }
        }
        arms
    }

    /// The sort a call of a word with an `at` list stands at, where
    /// `expected` is expected and `reaches` is the mode's Reaches(`expected`):
    /// the lowest sort of its `at` list that is at or above the lowest common
    /// sort of its arms. In body mode, when no such sort exists and the word
    /// stands at a test sort, the call stands there.
    fn at_list_sort(
        &self,
        call: &Call,
        expected: &SortKey,
        reaches: &[SortKey],
    ) -> Result<SortKey, NoSort> {
        let head = call.head.ident.as_str();
        let listed: Vec<&SortKey> = self
            .standing(head, reaches)
            .into_iter()
            .filter(|sort| self.is_at_list(head, sort))
            .collect();
        let signature = &self.registry[&(head.to_owned(), listed[0].clone())];
        let arms = Self::arms(call, &self.decls[signature.decl].params);
        if arms.is_empty() {
            return Err((
                call.span,
                RefusalKind::EmptyCoordination {
                    word: head.to_owned(),
                    sort: expected.to_string(),
                },
            ));
        }
        let mut arm_sorts = Vec::with_capacity(arms.len());
        for arm in arms {
            arm_sorts.push(self.arm_sort(arm, expected, reaches)?);
        }
        let mut common: Option<Vec<SortKey>> = None;
        for sort in &arm_sorts {
            let path = self
                .sorts
                .path(expected, sort, self.mode)
                .expect("an arm's sort is in Reaches of the expected sort");
            common = Some(match common {
                None => path,
                Some(common) => common
                    .into_iter()
                    .zip(path)
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect(),
            });
        }
        let common = common.expect("a call with arms has a common path");
        if let Some(sort) = common.iter().rev().find(|sort| listed.contains(sort)) {
            return Ok(sort.clone());
        }
        if self.mode == Mode::Body
            && let [test] = listed
                .iter()
                .filter(|sort| self.sorts.is_test(sort))
                .collect::<Vec<_>>()
                .as_slice()
        {
            return Ok((**test).clone());
        }
        Err((
            call.head.span,
            RefusalKind::NoCommonSort {
                word: head.to_owned(),
                sort: expected.to_string(),
                arms: arm_sorts.iter().map(ToString::to_string).collect(),
            },
        ))
    }

    /// The one sort of `reaches`, the mode's Reaches(`expected`), that the
    /// arm `value` stands at: its head word's, a parameter's declared sort (a
    /// list parameter passed whole giving its element sort), or for a nested
    /// call of a word with an `at` list the sort that call stands at.
    fn arm_sort(
        &self,
        value: &Value,
        expected: &SortKey,
        reaches: &[SortKey],
    ) -> Result<SortKey, NoSort> {
        let one = |standing: Vec<&SortKey>, none: RefusalKind| match standing.as_slice() {
            [sort] => Ok((*sort).clone()),
            [] => Err((value.span, none)),
            [..] => Err((
                value.span,
                RefusalKind::AmbiguousLiteral {
                    literal: "this".to_owned(),
                    sorts: standing.iter().map(ToString::to_string).collect(),
                    target: expected.to_string(),
                },
            )),
        };
        let at = expected.to_string();
        match &value.kind {
            ValueKind::Parameter(name) => {
                let name = name.ident.as_str();
                let Some(param) = self.params.iter().find(|p| p.name == name) else {
                    return Err((value.span, RefusalKind::UnknownParameter(name.to_owned())));
                };
                let sort = param.sort.option_inner().unwrap_or(&param.sort);
                let sort = sort.list_inner().unwrap_or(sort);
                if reaches.contains(sort) {
                    Ok(sort.clone())
                } else {
                    Err((
                        value.span,
                        RefusalKind::ParameterSort {
                            name: name.to_owned(),
                            sort: param.sort.to_string(),
                            expected: at,
                        },
                    ))
                }
            }
            ValueKind::Call(call) => {
                let head = call.head.ident.as_str();
                let standing = self.standing(head, reaches);
                if standing.iter().any(|sort| self.is_at_list(head, sort)) {
                    return self.at_list_sort(call, expected, reaches);
                }
                match standing.as_slice() {
                    [sort] => Ok((*sort).clone()),
                    [] => Err((call.head.span, self.unknown_word(head, expected))),
                    [..] => Err((
                        call.head.span,
                        RefusalKind::AmbiguousWord {
                            word: head.to_owned(),
                            sorts: standing.iter().map(ToString::to_string).collect(),
                            target: at,
                            mode: self.mode.name(),
                        },
                    )),
                }
            }
            ValueKind::String(_) => one(
                reaches
                    .iter()
                    .filter(|s| membership::string_stands_at(s))
                    .collect(),
                RefusalKind::StringAt(at),
            ),
            ValueKind::Integer(number) => one(
                reaches
                    .iter()
                    .filter(|s| membership::integer_stands_at(*number, s))
                    .collect(),
                RefusalKind::IntegerAt(at),
            ),
            ValueKind::List(_) => one(
                reaches
                    .iter()
                    .filter(|s| membership::list_stands_at(s))
                    .collect(),
                RefusalKind::ListAt(at),
            ),
            ValueKind::Record(_) => one(
                reaches
                    .iter()
                    .filter(|s| membership::record_stands_at(s))
                    .collect(),
                RefusalKind::RecordAt(at),
            ),
        }
    }

    /// Resolves the literal `value` at `expected`: at `expected` itself, at
    /// the sort `expected` is `Option` of, or at the one sort of the mode's
    /// Reaches(`expected`) it stands at.
    fn literal(&mut self, value: &Value, expected: &SortKey) {
        if let Some(inner) = expected.option_inner() {
            return self.literal(value, inner);
        }
        if !Self::stands_at(value, expected) {
            let reaches = self.sorts.reaches(expected, self.mode);
            let standing: Vec<&SortKey> = reaches
                .iter()
                .filter(|sort| Self::stands_at(value, sort))
                .collect();
            if let [sort] = standing.as_slice() {
                return self.literal(value, sort);
            }
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
            ValueKind::Call(_) | ValueKind::Parameter(_) => {
                unreachable!("a call or a parameter is no literal")
            }
        }
    }

    /// Whether the literal `value` stands at `sort` itself.
    fn stands_at(value: &Value, sort: &SortKey) -> bool {
        match &value.kind {
            ValueKind::String(_) => membership::string_stands_at(sort),
            ValueKind::Integer(number) => membership::integer_stands_at(*number, sort),
            ValueKind::List(_) => membership::list_stands_at(sort),
            ValueKind::Record(_) => membership::record_stands_at(sort),
            ValueKind::Call(_) | ValueKind::Parameter(_) => {
                unreachable!("a call or a parameter is no literal")
            }
        }
    }

    /// `$name` at `expected`: the parameter's own sort, `Option` of it, or a
    /// sort the parameter's sort reaches in the mode.
    fn parameter(&mut self, value: &Value, name: &str, expected: &SortKey) {
        let span = value.span;
        let Some(param) = self.params.iter().find(|p| p.name == name) else {
            return self.refuse(span, RefusalKind::UnknownParameter(name.to_owned()));
        };
        let sort = &param.sort;
        let at = expected.option_inner().unwrap_or(expected);
        if sort == expected || sort == at || self.sorts.reaches(at, self.mode).contains(sort) {
            return;
        }
        let kind = RefusalKind::ParameterSort {
            name: name.to_owned(),
            sort: sort.to_string(),
            expected: expected.to_string(),
        };
        self.refuse(span, kind);
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn plugin_bundles() -> (std::sync::Arc<Bundle>, std::sync::Arc<Bundle>) {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
        let canon = Bundle::load(plugins.join("canon"), builtin.next_file_id()).unwrap();
        (builtin, canon)
    }

    /// Every row of `schema::TEST_EXCEPTIONS` is used by its card, and every
    /// exception a card uses is a row.
    #[test]
    fn the_test_exceptions_are_exactly_the_ones_used() {
        let (builtin, canon) = plugin_bundles();
        let checked = load(&[&builtin, &canon]).unwrap_or_else(|error| panic!("{error}"));
        let rows: BTreeSet<(String, String)> = schema::TEST_EXCEPTIONS
            .iter()
            .map(|(card, test)| ((*card).to_owned(), (*test).to_owned()))
            .collect();
        assert_eq!(rows, checked.summary.test_exceptions);
    }

    /// Every row of the position tables names a declaration at its sort.
    #[test]
    fn every_position_table_row_names_a_declaration() {
        let (builtin, canon) = plugin_bundles();
        let checked = load(&[&builtin, &canon]).unwrap_or_else(|error| panic!("{error}"));
        let staying = schema::POSITION_STAYING
            .iter()
            .map(|(sort, word, _)| (*sort, *word));
        let missing: Vec<(&str, &str)> = schema::POSITION_OWN
            .iter()
            .copied()
            .chain(staying)
            .filter(|(sort, word)| {
                !checked
                    .registry
                    .contains_key(&((*word).to_owned(), SortKey::named(sort)))
            })
            .collect();
        assert_eq!(missing, []);
    }

    /// Every ticket the tables name is an open ticket.
    #[test]
    fn every_ticket_the_tables_name_is_open() {
        let tickets = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/tickets");
        let named: BTreeSet<&str> = schema::POSITION_STAYING
            .iter()
            .map(|(_, _, ticket)| *ticket)
            .chain([schema::TEST_EXCEPTION_TICKET])
            .collect();
        let missing: Vec<&str> = named
            .into_iter()
            .filter(|ticket| {
                !["planned", "critical", "maybe", "wip"]
                    .iter()
                    .any(|column| tickets.join(column).join(format!("{ticket}.md")).is_file())
            })
            .collect();
        assert_eq!(missing, Vec::<&str>::new());
    }

    /// Every sort the kind and record tables write is one the plugins
    /// produce and consume, so the tables name no misspelled sort.
    #[test]
    fn every_table_sort_is_known() {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
        let canon = Bundle::load(plugins.join("canon"), builtin.next_file_id()).unwrap();
        let decls: Vec<Decl<'_>> = [(true, &builtin), (false, &canon)]
            .into_iter()
            .flat_map(|(in_builtin, bundle)| {
                bundle
                    .files()
                    .values()
                    .flat_map(|file| file.ast.declarations())
                    .map(move |declaration| {
                        header(declaration, in_builtin, false).expect("the plugins' headers pass")
                    })
            })
            .collect();
        let registry = register(&decls, &mut Vec::new());
        let known = known_sorts(&decls, &registry);
        let mut written = Vec::new();
        for kind in KINDS {
            let implicit = kind.implicit.iter().map(|(_, sort)| *sort);
            let fields = kind.fields.iter().filter_map(|field| field.sort);
            written.extend(kind.sort.into_iter().chain(implicit).chain(fields));
            written.extend(kind.secondary.map(|secondary| secondary.sort));
        }
        for record in RECORDS {
            written.push(record.sort);
            written.extend(record.fields.iter().map(|(_, sort, _)| *sort));
        }
        written.extend([schema::TRIGGERED_ABILITY.0, schema::TRIGGER_CONDITION_SORT]);
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
