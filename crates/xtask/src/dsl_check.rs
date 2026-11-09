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
//! declare. Declaration kinds, record sorts and sort aliases have no `.syl`
//! declaration; their provisional schemas are [`schema`]'s.

mod schema;

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

use self::schema::KINDS;
use self::schema::Kind;
use self::schema::KindRule;
use self::schema::RECORDS;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summary {
    pub files: usize,
    /// Every declaration, cards included.
    pub declarations: usize,
    pub cards: usize,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} files, {} declarations ({} cards) resolve",
            self.files, self.declarations, self.cards
        )
    }
}

/// Why the bundles were refused.
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    /// `sylex_syl::check` refused them, so nothing was resolved.
    #[error(transparent)]
    Declarations(sylex_syl::check::CheckError),
    /// Every resolution refusal, each also rendered as `path:line:column:
    /// message`.
    #[error("{}", rendered.join("\n"))]
    Resolution {
        refusals: Vec<Refusal>,
        rendered: Vec<String>,
    },
}

impl CheckError {
    #[must_use]
    pub fn count(&self) -> usize {
        match self {
            CheckError::Declarations(error) => error.refusals.len(),
            CheckError::Resolution { refusals, .. } => refusals.len(),
        }
    }
}

/// One refused reference or declaration, located by its span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub span: Span,
    pub kind: RefusalKind,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusalKind {
    #[error("unknown declaration kind `{0}`")]
    UnknownKind(String),
    #[error("a `{0}` declaration is written with a name")]
    Nameless(String),
    #[error("a card is written without a name")]
    NamedCard,
    #[error("a card has no printed name: {0}")]
    CardName(String),
    #[error("`none` is the word for an omitted value and names no declaration")]
    NamedNone,
    #[error("a `{0}` declaration writes its result sort")]
    MissingSort(String),
    #[error("a `{kind}` declaration is declared at {sort}; it writes no sort")]
    WrittenSort { kind: String, sort: String },
    #[error("a `{0}` declaration has a `{{ ... }}` block")]
    MissingBlock(String),
    #[error("duplicate parameter `{0}`")]
    DuplicateParameter(String),
    #[error("sort `{0}` takes {1} sort arguments")]
    SortArity(String, usize),
    #[error("duplicate declaration `{name}` at {sort}")]
    DuplicateDeclaration { name: String, sort: String },
    #[error("{0} is reserved for expansion and registry declarations")]
    ReservedConstructor(String),
    #[error("duplicate field `{0}`")]
    DuplicateField(String),
    #[error("unknown field `{field}` at {at}")]
    UnknownField { field: String, at: String },
    #[error("missing field `{field}` at {at}")]
    MissingField { field: String, at: String },
    #[error("`identity` is a nonempty string on a parameterless NounPhrase template")]
    Identity,
    #[error("an action with `ofAbility` writes no `deed` or `permanent`")]
    OfAbilityWithDeed,
    #[error("an ability word needs the `abilityWord` declaration at Ability")]
    NoAbilityWordTemplate,
    #[error("unknown parameter `${0}`")]
    UnknownParameter(String),
    #[error("parameter `${name}` has sort {sort}, expected {expected}")]
    ParameterSort {
        name: String,
        sort: String,
        expected: String,
    },
    #[error("`none` stands only at an Option sort, bare; expected {0}")]
    NoneOutsideOption(String),
    #[error("`some` is never written; write the value")]
    SomeWritten,
    #[error("unknown word `{word}` at {sort}")]
    UnknownWord { word: String, sort: String },
    #[error("unknown argument `{argument}` to `{word}`")]
    UnknownArgument { word: String, argument: String },
    #[error("too many arguments to `{0}`")]
    TooManyArguments(String),
    #[error("duplicate argument `{argument}` to `{word}`")]
    DuplicateArgument { word: String, argument: String },
    #[error("missing argument `{argument}` to `{word}`")]
    MissingArgument { word: String, argument: String },
    #[error("string at {0}")]
    StringAt(String),
    #[error("integer at {0}")]
    IntegerAt(String),
    #[error("negative integer at Nat")]
    NegativeAtNat,
    #[error("list at {0}")]
    ListAt(String),
    #[error("record at {0}, which is not a record sort")]
    RecordAt(String),
    #[error("dependency cycle: {}", .0.join(" -> "))]
    Cycle(Vec<String>),
}

/// A sort with its aliases unfolded: `Option(List(ManaSymbol))`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SortKey {
    head: String,
    args: Vec<SortKey>,
}

/// The sort constructors, each taking one sort argument.
const SORT_CONSTRUCTORS: [&str; 3] = ["Option", "List", "Delta"];

impl SortKey {
    fn named(head: &str) -> SortKey {
        SortKey {
            head: head.to_owned(),
            args: Vec::new(),
        }
    }

    /// Reads a schema table's sort, `Option(List(ManaSymbol))`.
    fn table(text: &str) -> SortKey {
        let (key, rest) = SortKey::table_prefix(text);
        assert!(rest.is_empty(), "malformed schema sort `{text}`");
        key
    }

    fn table_prefix(text: &str) -> (SortKey, &str) {
        let end = text.find(['(', ')', ',']).unwrap_or(text.len());
        let mut key = SortKey::named(text[..end].trim());
        let mut rest = &text[end..];
        if let Some(inner) = rest.strip_prefix('(') {
            rest = inner;
            loop {
                let (arg, after) = SortKey::table_prefix(rest.trim_start());
                key.args.push(arg);
                if let Some(after) = after.strip_prefix(',') {
                    rest = after;
                } else {
                    rest = after.strip_prefix(')').expect("schema sort closes");
                    break;
                }
            }
        }
        (key, rest)
    }

    fn option_inner(&self) -> Option<&SortKey> {
        (self.head == "Option").then(|| &self.args[0])
    }

    fn list_inner(&self) -> Option<&SortKey> {
        (self.head == "List").then(|| &self.args[0])
    }
}

impl fmt::Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.head)?;
        if !self.args.is_empty() {
            f.write_str("(")?;
            for (index, arg) in self.args.iter().enumerate() {
                if index > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{arg}")?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

/// A written sort with its aliases unfolded, refusing a wrong number of sort
/// arguments.
fn sort_key(sort: &Sort) -> Result<SortKey, Refusal> {
    let head = sort.head.ident.as_str();
    let arity = usize::from(SORT_CONSTRUCTORS.contains(&head));
    if sort.args.len() != arity {
        return Err(Refusal {
            span: sort.span,
            kind: RefusalKind::SortArity(head.to_owned(), arity),
        });
    }
    if let Some((_, target)) = schema::ALIASES.iter().find(|(alias, _)| *alias == head) {
        return Ok(SortKey::table(target));
    }
    Ok(SortKey {
        head: head.to_owned(),
        args: sort.args.iter().map(sort_key).collect::<Result<_, _>>()?,
    })
}

/// A parameter of a declaration, or a field of a record or block.
#[derive(Debug, Clone)]
struct Slot {
    name: String,
    /// `None`: written but not resolved, for the one temporary exception that
    /// needs it (`subtype`'s `kind = spell`).
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
    /// The reserved constructor a primitive builds, as `Sort.Constructor`.
    reserved: Option<String>,
}

type Registry = BTreeMap<(String, SortKey), Signature>;

/// The file a temporary exception lets build the reserved `Action.OwnScope`,
/// relative to the builtin bundle.
const RING_FILE: &str = "definitions/keyword_action/theRingTemptsYou.syl";

/// One declaration whose header passed.
struct Decl<'a> {
    declaration: &'a Declaration,
    kind: &'static Kind,
    name: String,
    sort: SortKey,
    params: Vec<Slot>,
    /// Whether this is the file `RING_FILE` names.
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
    let ring_file = builtin.root().join(RING_FILE);
    let mut refusals = Vec::new();
    let mut decls = Vec::new();
    let mut summary = Summary {
        files: 0,
        declarations: 0,
        cards: 0,
    };
    for bundle in &bundles {
        for file in bundle.files().values() {
            summary.files += 1;
            let ring = std::ptr::eq(*bundle, builtin) && file.path == ring_file;
            for declaration in file.ast.declarations() {
                summary.declarations += 1;
                if declaration.kind.ident == schema::CARD {
                    summary.cards += 1;
                }
                match header(declaration, ring) {
                    Ok(decl) => decls.push(decl),
                    Err(refusal) => refusals.push(refusal),
                }
            }
        }
    }
    let registry = register(&decls, &mut refusals);
    let mut edges = vec![Vec::new(); decls.len()];
    for (decl, edges) in decls.iter().zip(&mut edges) {
        resolve(decl, &registry, &mut refusals, edges);
    }
    cycles(&decls, &edges, &mut refusals);
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
    let named = match &declaration.shape {
        Shape::Nameless(value) if word == schema::CARD => {
            return match crate::dsl::printed_name(value) {
                Ok(name) => Ok(decl(name, SortKey::named("Card"), Vec::new())),
                Err(message) => refuse(value.span, RefusalKind::CardName(message)),
            };
        }
        Shape::Nameless(_) => {
            return refuse(declaration.span, RefusalKind::Nameless(word.to_owned()));
        }
        Shape::Named(_) if word == schema::CARD => {
            return refuse(declaration.span, RefusalKind::NamedCard);
        }
        Shape::Named(named) => named,
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
    if named.body.is_none() && word != "primitive" {
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
    Ok(decl(name.to_owned(), sort, params))
}

/// The reserved constructor a primitive at `sort` naming `constructor`
/// builds, as `Sort.Constructor`: one only expansion writes (`syl.md` §1 and
/// §4: a scope, a mention, a caller-scope wrapper, a raw Deed, a keyword
/// wrapper), or any constructor of a sort only declaration kinds build.
fn reserved(sort: &SortKey, constructor: &str) -> Option<String> {
    let expansion = constructor == "InCaller"
        || matches!(
            (sort.head.as_str(), constructor),
            ("NounPhrase", "FirstMention" | "LaterMention")
                | ("Action", "OwnScope")
                | ("Deed", "Action")
                | ("Ability", "Keyword")
        );
    let registry_sort = ["KeywordLabel", "RegistryDefinition"].contains(&sort.head.as_str())
        || KINDS
            .iter()
            .any(|kind| kind.catalog && kind.sort == Some(sort.head.as_str()));
    (expansion || registry_sort).then(|| format!("{sort}.{constructor}"))
}

/// Temporary: `theRingTemptsYou.syl` declares and calls a primitive building
/// the reserved `Action.OwnScope`, and only that file may.
/// `syl-ring-tempts-you-raw-scope` deletes this exception.
fn ring_exception(ring: bool, reserved: &str) -> bool {
    ring && reserved == "Action.OwnScope"
}

/// Registers every declaration at its `(name, sort)` and its second sort,
/// refusing duplicates and primitives that build a reserved constructor.
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
        let constructor = match &decl.declaration.shape {
            Shape::Named(named) => named.constructor,
            Shape::Nameless(_) => None,
        };
        let reserved = constructor.and_then(|c| reserved(&decl.sort, c.ident.as_str()));
        if let (Some(reserved), Some(constructor)) = (&reserved, constructor)
            && !ring_exception(decl.ring, reserved)
        {
            refusals.push(Refusal {
                span: constructor.span,
                kind: RefusalKind::ReservedConstructor(reserved.clone()),
            });
        }
        let signature = Signature {
            params: decl.params.clone(),
            node: (!decl.kind.catalog).then_some(index),
            reserved,
        };
        let span = decl.declaration.span;
        insert(
            (decl.name.clone(), decl.sort.clone()),
            signature,
            span,
            refusals,
        );
        let Some(secondary) = decl.kind.secondary else {
            continue;
        };
        // An action names no Deed when it is an ability's (`ofAbility`) or
        // writes `deed = none`.
        let deedless = decl.field("ofAbility").is_some()
            || decl.field("deed").is_some_and(|deed| is_bare(deed, "none"));
        if secondary == "Deed" && deedless {
            continue;
        }
        let signature = Signature {
            params: Vec::new(),
            node: None,
            reserved: None,
        };
        insert(
            (decl.name.clone(), SortKey::named(secondary)),
            signature,
            span,
            refusals,
        );
    }
    registry
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
        if let Shape::Nameless(value) = &decl.declaration.shape {
            self.value(value, &decl.sort);
            return;
        }
        // Only a primitive may omit its block, and then it has nothing to
        // resolve.
        let Some(block) = decl.block() else {
            return;
        };
        let at = format!("`{}` {}", decl.kind.word, decl.name);
        if decl.kind.record_fields && decl.field("body").is_none() {
            self.record(block, &decl.sort);
            return;
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
        match decl.kind.rule {
            KindRule::None => {}
            KindRule::Identity => {
                if let Some(identity) = decl.field("identity") {
                    let nonempty =
                        matches!(&identity.kind, ValueKind::String(text) if !text.is_empty());
                    if !nonempty || decl.sort.head != "NounPhrase" || !decl.params.is_empty() {
                        self.refuse(identity.span, RefusalKind::Identity);
                    }
                }
            }
            KindRule::OfAbility => {
                if let Some(of_ability) = decl.field("ofAbility")
                    && (decl.field("deed").is_some() || decl.field("permanent").is_some())
                {
                    self.refuse(of_ability.span, RefusalKind::OfAbilityWithDeed);
                }
            }
            KindRule::AbilityWord => {
                let key = ("abilityWord".to_owned(), SortKey::named("Ability"));
                match self.registry.get(&key) {
                    Some(signature) => self.edges.extend(signature.node),
                    None => self.refuse(decl.declaration.span, RefusalKind::NoAbilityWordTemplate),
                }
            }
            KindRule::SpellKind => {
                if decl
                    .field("kind")
                    .is_some_and(|kind| is_bare(kind, "spell"))
                {
                    let kind = slots.iter_mut().find(|slot| slot.name == "kind");
                    kind.expect("a subtype has a kind field").sort = None;
                }
            }
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
        if let Some(reserved) = &signature.reserved
            && !ring_exception(self.ring, reserved)
        {
            self.refuse(
                call.head.span,
                RefusalKind::ReservedConstructor(reserved.clone()),
            );
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

/// Refuses every dependency cycle: a set of declarations each reaching the
/// next, or one reaching itself.
fn cycles(decls: &[Decl<'_>], edges: &[Vec<usize>], refusals: &mut Vec<Refusal>) {
    for component in strongly_connected(edges) {
        let first = component[0];
        if component.len() > 1 || edges[first].contains(&first) {
            let names = component.iter().map(|&i| decls[i].name.clone()).collect();
            refusals.push(Refusal {
                span: decls[first].declaration.span,
                kind: RefusalKind::Cycle(names),
            });
        }
    }
}

/// Tarjan's strongly connected components, each sorted.
fn strongly_connected(edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct State<'e> {
        edges: &'e [Vec<usize>],
        index: Vec<Option<usize>>,
        low: Vec<usize>,
        stack: Vec<usize>,
        on_stack: Vec<bool>,
        next: usize,
        out: Vec<Vec<usize>>,
    }
    fn visit(s: &mut State<'_>, v: usize) {
        s.index[v] = Some(s.next);
        s.low[v] = s.next;
        s.next += 1;
        s.stack.push(v);
        s.on_stack[v] = true;
        for &w in &s.edges[v] {
            match s.index[w] {
                None => {
                    visit(s, w);
                    s.low[v] = s.low[v].min(s.low[w]);
                }
                Some(index) if s.on_stack[w] => s.low[v] = s.low[v].min(index),
                Some(_) => {}
            }
        }
        if Some(s.low[v]) == s.index[v] {
            let mut component = Vec::new();
            loop {
                let w = s.stack.pop().expect("v is on the stack");
                s.on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            component.sort_unstable();
            s.out.push(component);
        }
    }
    let n = edges.len();
    let mut state = State {
        edges,
        index: vec![None; n],
        low: vec![0; n],
        stack: Vec::new(),
        on_stack: vec![false; n],
        next: 0,
        out: Vec::new(),
    };
    for v in 0..n {
        if state.index[v].is_none() {
            visit(&mut state, v);
        }
    }
    state.out
}
