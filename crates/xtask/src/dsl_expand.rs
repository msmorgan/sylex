//! `cargo xtask dsl expand`: the expansion of one card or one term.
//!
//! A throwaway, built ahead of the Rust AST for the card language. When the
//! proper expansion on that AST exists, this module is deleted and its
//! property tests are rewritten against that target: their content is what
//! lasts, not this code.
//!
//! It is expansion-lite (`docs/design/syl.md` §4). It reads what
//! [`crate::dsl_check`]'s resolver resolved and substitutes each Template and
//! Definition call by its body with the arguments in place, binds an `action`
//! Definition's implicit `$actor` to the call's performer, fills declared
//! defaults and applies coercions. It keeps one kind of boundary: a
//! [`Node::Mark`] where a Definition was entered, carrying the Deed its
//! declaration states, possibly none. Template, parameter-binding and coercion
//! boundaries are not kept, and nothing of binding and re-mention, scopes or
//! referents is modeled. Where §4 is silent it takes the plain structural
//! reading: a default is expanded where the callee declares it, reading the
//! callee's earlier parameters; an unused parameter's argument appears nowhere;
//! a value written bare at an `Option` position is that value, with no `some`.
//! Two expansions are the same content when they are equal as [`Node`] trees.
//!
//! A term is expanded as the body of a Template the command writes around it,
//! [`TERM`], whose parameters are the term's free values: `$x` stays
//! [`Node::Param`], an opaque value.

mod tree;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use anyhow::anyhow;
use anyhow::bail;
use clap::Args;
use clap::Subcommand;
use sylex_syl::Bundle;
use sylex_syl::Call;
use sylex_syl::FileId;
use sylex_syl::Header;
use sylex_syl::Item;
use sylex_syl::Shape;
use sylex_syl::Value;
use sylex_syl::ValueKind;

pub use self::tree::Deed;
pub use self::tree::Node;
use crate::dsl_check::Checked;
use crate::dsl_check::Decl;
use crate::dsl_check::SortKey;

#[derive(Debug, Args)]
pub struct ExpandArgs {
    #[command(subcommand)]
    what: What,
}

#[derive(Debug, Subcommand)]
enum What {
    /// A canon card, by its file's name (`Abrupt Decay`).
    Card { name: String },
    /// A term at a sort, its free values declared as parameters:
    /// `term 'destroy($x)' --at VerbPhrase --param x:NounPhrase`.
    Term {
        term: String,
        /// The sort the term stands at.
        #[arg(long)]
        at: String,
        /// A free value `name:Sort`, read in the term as `$name`.
        #[arg(long = "param")]
        params: Vec<String>,
    },
}

/// The name of the Template written around a term.
pub const TERM: &str = "expandedTerm";

pub fn run(args: &ExpandArgs) -> anyhow::Result<()> {
    match &args.what {
        What::Card { name } => {
            let bundles = bundles(Vec::new())?;
            let checked = check(&bundles)?;
            let expander = Expander::new(&checked);
            let mut found = None;
            for decl in checked.decls.iter().filter(|decl| decl.kind.word == "card") {
                if card_name(&bundles, decl)? == *name {
                    found = Some(decl);
                    break;
                }
            }
            let decl = found.ok_or_else(|| anyhow!("no card file is named `{name}`"))?;
            println!("{}", expander.card(decl)?);
        }
        What::Term { term, at, params } => {
            let params: Vec<(&str, &str)> = params
                .iter()
                .map(|param| {
                    param
                        .split_once(':')
                        .map(|(name, sort)| (name.trim(), sort.trim()))
                        .ok_or_else(|| anyhow!("a parameter is `name:Sort`, not `{param}`"))
                })
                .collect::<anyhow::Result<_>>()?;
            let bundles = bundles(vec![probe(TERM, &params, at, term)])?;
            let checked = check(&bundles)?;
            println!("{}", Expander::new(&checked).term(TERM, at)?);
        }
    }
    Ok(())
}

/// The file name a card's declaration is read from, without `.syl`.
fn card_name(bundles: &[Arc<Bundle>], decl: &Decl<'_>) -> anyhow::Result<String> {
    let file = bundles
        .iter()
        .find_map(|bundle| bundle.file(decl.declaration.span.file))
        .ok_or_else(|| anyhow!("a card's declaration lies in no loaded bundle's file"))?;
    let stem = file
        .path
        .file_stem()
        .ok_or_else(|| anyhow!("card file `{}` has no name", file.path.display()))?;
    Ok(stem.to_string_lossy().into_owned())
}

/// A `template` declaration's source: `body` at `sort`, reading `params`.
#[must_use]
pub fn probe(name: &str, params: &[(&str, &str)], sort: &str, body: &str) -> (PathBuf, String) {
    let params = if params.is_empty() {
        String::new()
    } else {
        let params: Vec<String> = params
            .iter()
            .map(|(name, sort)| format!("{name}: {sort}"))
            .collect();
        format!("({})", params.join(", "))
    };
    (
        PathBuf::from(format!("{name}.syl")),
        format!("template {name}{params}: {sort} {{\n  body = {body}\n}}\n"),
    )
}

/// The plugin bundles, builtin first, then canon, then `probes` as one more
/// bundle when there are any.
pub fn bundles(probes: Vec<(PathBuf, String)>) -> anyhow::Result<Vec<Arc<Bundle>>> {
    let plugins = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../plugins")
        .canonicalize()?;
    let builtin = Bundle::load(plugins.join("builtin"), FileId(0))?;
    let canon = Bundle::load(plugins.join("canon"), builtin.next_file_id())?;
    let mut bundles = vec![builtin, canon];
    if !probes.is_empty() {
        let first = bundles[1].next_file_id();
        bundles.push(Bundle::from_sources("<probe>", probes, first)?);
    }
    Ok(bundles)
}

/// Checks `bundles`, refusing to expand anything `dsl check` refuses.
pub(crate) fn check(bundles: &[Arc<Bundle>]) -> anyhow::Result<Checked<'_>> {
    let refs: Vec<&Bundle> = bundles.iter().map(AsRef::as_ref).collect();
    crate::dsl_check::load(&refs).map_err(|error| anyhow!("dsl check refused:\n{error}"))
}

/// What a declaration's kind does when its word is called.
enum Role {
    /// Builds its constructor.
    Primitive,
    /// Is replaced by its body, leaving no boundary: a Template, a coercion,
    /// and an ability word, which `surface.md` §6 lists among no
    /// Definition.
    Template,
    /// Is replaced by its body inside a mark (`surface.md` §6).
    Definition,
    /// Names a declaration and calls nothing: a catalog declaration.
    Named,
}

fn role(kind: &str) -> anyhow::Result<Role> {
    Ok(match kind {
        "primitive" => Role::Primitive,
        "template" | "coercion" | "ability_word" => Role::Template,
        "action" | "ability" | "token" => Role::Definition,
        "counter" | "designation" | "subtype" | "card_type" | "supertype" | "relation"
        | "verb" | "sba" | "conferral" => Role::Named,
        other => bail!("a `{other}` declaration has no word to call"),
    })
}

/// A parameter's binding inside a body.
#[derive(Clone)]
enum Binding<'a> {
    /// An expanded value: `$actor`, a coercion's input, an earlier primitive
    /// parameter, or a free parameter in `term` or `fields`.
    Node(Node),
    /// A parameter's argument or default, with the scope and performer at the
    /// site that supplied it. A `VerbPhrase` parameter has no subject of its
    /// own, so it takes the performer where the body places it.
    Deferred {
        value: &'a Value,
        sort: SortKey,
        env: Env<'a>,
        performer: Node,
    },
}

/// The parameters' bindings inside a body.
type Env<'a> = BTreeMap<String, Binding<'a>>;

/// Expands values against one check's resolutions.
pub struct Expander<'c, 'a> {
    checked: &'c Checked<'a>,
}

impl<'c, 'a> Expander<'c, 'a> {
    pub(crate) fn new(checked: &'c Checked<'a>) -> Self {
        Expander { checked }
    }

    /// The declaration of `name` at `sort`.
    pub(crate) fn declaration(&self, name: &str, sort: &str) -> anyhow::Result<&'c Decl<'a>> {
        let signature = self
            .checked
            .registry
            .get(&(name.to_owned(), SortKey::table(sort)))
            .ok_or_else(|| anyhow!("no declaration `{name}` at `{sort}`"))?;
        Ok(&self.checked.decls[signature.decl])
    }

    /// A Template's body with each parameter free.
    pub fn term(&self, name: &str, sort: &str) -> anyhow::Result<Node> {
        let decl = self.declaration(name, sort)?;
        self.value(body(decl)?, &free(decl), &ambient_performer())
    }

    /// A card's expansion.
    pub(crate) fn card(&self, decl: &Decl<'a>) -> anyhow::Result<Node> {
        let Shape::Value(value) = &decl.declaration.shape else {
            bail!("a card is a value");
        };
        self.value(value, &Env::new(), &ambient_performer())
    }

    /// Each field of a declaration's block but its metadata, expanded with
    /// each parameter free.
    #[cfg(test)]
    pub(crate) fn fields(&self, decl: &Decl<'a>) -> anyhow::Result<Vec<(String, Node)>> {
        let env = free(decl);
        let performer = ambient_performer();
        let Some(block) = decl.block() else {
            return Ok(Vec::new());
        };
        block
            .nodes()
            .filter(|field| !sylex_syl::METADATA_FIELDS.contains(&field.name.ident.as_str()))
            .map(|field| {
                let name = field.name.ident.as_str().to_owned();
                self.value(&field.value, &env, &performer)
                    .with_context(|| format!("field `{name}`"))
                    .map(|node| (name, node))
            })
            .collect()
    }

    /// The Deed `decl` states, if it is a Definition that states one.
    pub(crate) fn deed(decl: &Decl<'_>) -> anyhow::Result<Option<Deed>> {
        if decl.kind.word != "action" {
            return Ok(None);
        }
        let name = decl.name.as_deref().expect("an action is named");
        match (decl.field("deed"), decl.field("ofAbility")) {
            (None, None) => Ok(Some(Deed::Label(name.to_owned()))),
            (Some(deed), None) => match &deed.kind {
                ValueKind::String(label) => Ok(Some(Deed::Label(label.clone()))),
                ValueKind::Call(call) if call.is_bare() && call.head.ident == "none" => Ok(None),
                ValueKind::Call(_)
                | ValueKind::Integer(_)
                | ValueKind::List(_)
                | ValueKind::Record(_)
                | ValueKind::Parameter(_) => bail!("`{name}`'s `deed` is neither a label nor none"),
            },
            (None, Some(ability)) => match &ability.kind {
                ValueKind::Call(call) if call.is_bare() => {
                    Ok(Some(Deed::OfAbility(call.head.ident.as_str().to_owned())))
                }
                ValueKind::Call(_)
                | ValueKind::String(_)
                | ValueKind::Integer(_)
                | ValueKind::List(_)
                | ValueKind::Record(_)
                | ValueKind::Parameter(_) => bail!("`{name}`'s `ofAbility` is no bare word"),
            },
            (Some(_), Some(_)) => bail!("`{name}` writes both `deed` and `ofAbility`"),
        }
    }

    /// Expands `value`, whose parameters read `env`, under `performer`.
    fn value(
        &self,
        value: &'a Value,
        env: &Env<'a>,
        performer: &Node,
    ) -> anyhow::Result<Node> {
        let node = match &value.kind {
            ValueKind::Parameter(name) => {
                let name = name.ident.as_str();
                match env
                    .get(name)
                    .ok_or_else(|| anyhow!("`${name}` reads no parameter"))?
                {
                    Binding::Node(node) => node.clone(),
                    Binding::Deferred {
                        value,
                        sort,
                        env,
                        performer: caller_performer,
                    } => {
                        let active = if is_verb_phrase_sort(sort) {
                            performer
                        } else {
                            caller_performer
                        };
                        self.value(value, env, active)?
                    }
                }
            }
            ValueKind::String(text) => Node::String(text.clone()),
            ValueKind::Integer(number) => Node::Integer(*number),
            ValueKind::List(items) => Node::List(
                items
                    .iter()
                    .filter_map(Item::node)
                    .map(|item| self.value(item, env, performer))
                    .collect::<anyhow::Result<_>>()?,
            ),
            ValueKind::Record(record) => self.record(record, env, performer)?,
            ValueKind::Call(call) => self.call(call, env, performer)?,
        };
        let Some(index) = self.checked.resolutions.coerced(value) else {
            return Ok(node);
        };
        let coercion = &self.checked.decls[index];
        let [param] = coercion.params.as_slice() else {
            bail!("a coercion has one parameter");
        };
        self.value(
            body(coercion)?,
            &Env::from([(param.name.clone(), Binding::Node(node))]),
            performer,
        )
    }

    fn record(
        &self,
        record: &'a sylex_syl::Record,
        env: &Env<'a>,
        performer: &Node,
    ) -> anyhow::Result<Node> {
        let mut fields: Vec<(String, Node)> = Vec::new();
        if let Some(subj_field) = record.nodes().find(|f| f.name.ident == "subj") {
            let subj = self.value(&subj_field.value, env, performer)?;
            let clause_performer = subject_performer(&subj)?;
            fields.push(("subj".to_owned(), subj));
            for field in record.nodes().filter(|f| f.name.ident != "subj") {
                let name = field.name.ident.as_str();
                let active = if name == "action" {
                    &clause_performer
                } else {
                    performer
                };
                fields.push((name.to_owned(), self.value(&field.value, env, active)?));
            }
        } else {
            for field in record.nodes() {
                let node = self.value(&field.value, env, performer)?;
                fields.push((field.name.ident.as_str().to_owned(), node));
            }
        }
        fields.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(Node::Record(fields))
    }

    fn call(&self, call: &'a Call, env: &Env<'a>, performer: &Node) -> anyhow::Result<Node> {
        let word = call.head.ident.as_str();
        if word == "none" && call.is_bare() {
            return Ok(Node::None);
        }
        let sort = self
            .checked
            .resolutions
            .call(call)
            .ok_or_else(|| anyhow!("`{word}` was never resolved"))?;
        let signature = &self.checked.registry[&(word.to_owned(), sort.clone())];
        let decl = &self.checked.decls[signature.decl];
        let sort_name = sort.to_string();
        if signature.secondary {
            return Ok(Node::Word {
                word: word.to_owned(),
                sort: sort_name,
            });
        }
        let expand = || -> anyhow::Result<Node> {
            Ok(match role(decl.kind.word)? {
                Role::Named => Node::Word {
                    word: word.to_owned(),
                    sort: sort_name,
                },
                Role::Primitive => {
                    let constructor = header(decl)?
                        .constructor
                        .ok_or_else(|| anyhow!("primitive `{word}` names no constructor"))?;
                    let constructor_name = constructor.ident.as_str();
                    let args = self.primitive_args(
                        call,
                        decl,
                        constructor_name,
                        &sort_name,
                        env,
                        performer,
                    )?;
                    Node::Call {
                        constructor: constructor_name.to_owned(),
                        word: word.to_owned(),
                        sort: sort_name,
                        args,
                    }
                }
                Role::Template => {
                    let callee = self.bind(call, decl, env, performer)?;
                    self.value(body(decl)?, &callee, performer)?
                }
                Role::Definition => {
                    let mut callee = self.bind(call, decl, env, performer)?;
                    for (implicit, _) in decl.kind.implicit {
                        callee.insert((*implicit).to_owned(), Binding::Node(performer.clone()));
                    }
                    let inner_performer = if sort_name == "Ability" {
                        ambient_performer()
                    } else {
                        performer.clone()
                    };
                    Node::Mark {
                        definition: word.to_owned(),
                        sort: sort_name,
                        deed: Self::deed(decl)?,
                        body: Box::new(self.value(body(decl)?, &callee, &inner_performer)?),
                    }
                }
            })
        };
        expand().with_context(|| format!("expanding `{word}` at `{sort}`"))
    }

    fn written_args<'b>(&self, call: &'b Call, arity: usize) -> anyhow::Result<Vec<Option<&'b Value>>> {
        let mut written = vec![None; arity];
        for arg in call.args.iter().filter_map(Item::node) {
            let index = self
                .checked
                .resolutions
                .arg(arg)
                .ok_or_else(|| anyhow!("an argument of `{}` binds nothing", call.head.ident))?;
            written[index] = Some(&arg.value);
        }
        Ok(written)
    }

    /// Expands a primitive's arguments in declaration order, updating the
    /// active performer across `Instruct`, `Subject`-bearing primitives and
    /// `Ability` roots.
    fn primitive_args(
        &self,
        call: &'a Call,
        decl: &Decl<'a>,
        constructor: &str,
        sort_name: &str,
        env: &Env<'a>,
        performer: &Node,
    ) -> anyhow::Result<Vec<Node>> {
        let base = if sort_name == "Ability" {
            ambient_performer()
        } else {
            performer.clone()
        };
        let written = self.written_args(call, decl.params.len())?;
        if constructor == "Instruct" && sort_name == "VerbPhrase" {
            let [Some(subj_val), Some(action_val)] = written.as_slice() else {
                bail!("`instruct` takes `subj` and `action`");
            };
            let subj = self.value(subj_val, env, &base)?;
            let action = self.value(action_val, env, &subj)?;
            return Ok(vec![subj, action]);
        }
        let header_params: Vec<_> = header(decl)?.params.iter().filter_map(Item::node).collect();
        let subj_clause = decl
            .params
            .iter()
            .position(|param| param.sort == SortKey::table("Subject"))
            .map(|index| -> anyhow::Result<(usize, Node, Node)> {
                let value = written[index]
                    .ok_or_else(|| anyhow!("`{}` has no `Subject` argument", call.head.ident))?;
                let subj = self.value(value, env, &base)?;
                let clause_performer = subject_performer(&subj)?;
                Ok((index, subj, clause_performer))
            })
            .transpose()?;
        let mut callee = Env::new();
        let mut out = Vec::with_capacity(decl.params.len());
        for (index, ((param, slot), written)) in header_params
            .into_iter()
            .zip(&decl.params)
            .zip(written)
            .enumerate()
        {
            let name = param.name.ident.as_str();
            let node = if let Some((subj_index, subj, _)) = &subj_clause
                && index == *subj_index
            {
                subj.clone()
            } else {
                let active = match &subj_clause {
                    Some((_, _, clause_performer))
                        if is_verb_phrase_sort(&slot.sort)
                            || slot.sort == SortKey::table("DeonticMods") =>
                    {
                        clause_performer
                    }
                    _ => &base,
                };
                match (written, &param.default) {
                    (Some(value), _) => self.value(value, env, active)?,
                    (None, Some(default)) => self.value(default, &callee, active)?,
                    (None, None) => bail!("`${name}` has no argument and no default"),
                }
            };
            callee.insert(name.to_owned(), Binding::Node(node.clone()));
            out.push(node);
        }
        Ok(out)
    }

    /// Each parameter of `decl` bound to its argument in `call`, in the
    /// caller's `env`, or to its default, reading the parameters before it.
    fn bind(
        &self,
        call: &'a Call,
        decl: &Decl<'a>,
        env: &Env<'a>,
        performer: &Node,
    ) -> anyhow::Result<Env<'a>> {
        let written = self.written_args(call, decl.params.len())?;
        let mut callee = Env::new();
        for ((param, slot), written) in header(decl)?
            .params
            .iter()
            .filter_map(Item::node)
            .zip(&decl.params)
            .zip(written)
        {
            let name = param.name.ident.as_str();
            let binding = match (written, &param.default) {
                (Some(value), _) => Binding::Deferred {
                    value,
                    sort: slot.sort.clone(),
                    env: env.clone(),
                    performer: performer.clone(),
                },
                (None, Some(default)) => Binding::Deferred {
                    value: default,
                    sort: slot.sort.clone(),
                    env: callee.clone(),
                    performer: performer.clone(),
                },
                (None, None) => bail!("`${name}` has no argument and no default"),
            };
            callee.insert(name.to_owned(), binding);
        }
        Ok(callee)
    }
}

/// The performer of an instruction or unplaced verb phrase before a Handoff or
/// `Subject` names one.
fn ambient_performer() -> Node {
    Node::Call {
        constructor: "Actor".to_owned(),
        word: "actor".to_owned(),
        sort: "NounPhrase".to_owned(),
        args: Vec::new(),
    }
}

fn is_verb_phrase_sort(sort: &SortKey) -> bool {
    *sort == SortKey::table("VerbPhrase")
        || *sort == SortKey::table("Option(VerbPhrase)")
        || *sort == SortKey::table("List(VerbPhrase)")
}

fn subject_performer(subj: &Node) -> anyhow::Result<Node> {
    match subj {
        Node::Call {
            constructor,
            sort,
            args,
            ..
        } if sort == "Subject" => match (constructor.as_str(), args.as_slice()) {
            ("Actor" | "By", [np]) => Ok(np.clone()),
            ("Any", []) => Ok(Node::UnnamedPerformer),
            (other, _) => bail!("unexpected Subject constructor `{other}`"),
        },
        Node::Param(name) => Ok(Node::Param(format!("{name}.actor"))),
        other => bail!("expected a Subject node, got {other}"),
    }
}

fn header<'a>(decl: &Decl<'a>) -> anyhow::Result<&'a Header> {
    match &decl.declaration.shape {
        Shape::Header(header) => Ok(header),
        Shape::Value(_) => bail!("a value declaration has no header"),
    }
}

fn body<'a>(decl: &Decl<'a>) -> anyhow::Result<&'a Value> {
    decl.field("body")
        .ok_or_else(|| anyhow!("{} has no body to expand", decl.label()))
}

/// Each of `decl`'s parameters, explicit and implicit, free.
fn free<'a>(decl: &Decl<'a>) -> Env<'a> {
    decl.params
        .iter()
        .map(|param| param.name.clone())
        .chain(decl.kind.implicit.iter().map(|(name, _)| (*name).to_owned()))
        .map(|name| {
            let param = Node::Param(name.clone());
            (name, Binding::Node(param))
        })
        .collect()
}
