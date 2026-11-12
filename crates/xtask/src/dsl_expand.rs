//! `cargo xtask dsl expand`: the expansion of one card or one term.
//!
//! A throwaway, built ahead of the Rust AST for the card language. When the
//! proper expansion on that AST exists, this module is deleted and its
//! property tests are rewritten against that target: their content is what
//! lasts, not this code.
//!
//! It is expansion-lite (`docs/decisions/syl.md` §4). It reads what
//! [`crate::dsl_check`]'s resolver resolved and substitutes each Template and
//! Definition call by its body with the arguments in place, fills declared
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
    /// `term 'destroy($x)' --at Action --param x:NounPhrase`.
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
            let decl = checked
                .decls
                .iter()
                .find(|decl| {
                    decl.kind.word == "card"
                        && card_name(&bundles, decl).is_some_and(|stem| stem == *name)
                })
                .ok_or_else(|| anyhow!("no card file is named `{name}`"))?;
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
fn card_name(bundles: &[Arc<Bundle>], decl: &Decl<'_>) -> Option<String> {
    let file = bundles
        .iter()
        .find_map(|bundle| bundle.file(decl.declaration.span.file))?;
    Some(file.path.file_stem()?.to_string_lossy().into_owned())
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
    /// and an ability word, which `semantics.md` §3 lists among no
    /// Definition.
    Template,
    /// Is replaced by its body inside a mark (`semantics.md` §3).
    Definition,
    /// Names a declaration and calls nothing: a catalog declaration.
    Named,
}

fn role(kind: &str) -> anyhow::Result<Role> {
    Ok(match kind {
        "primitive" => Role::Primitive,
        "template" | "coercion" | "ability_word" => Role::Template,
        "action" | "ability" | "token" => Role::Definition,
        "counter" | "designation" | "subtype" | "card_type" | "supertype" | "relation" | "sba"
        | "conferral" => Role::Named,
        other => bail!("a `{other}` declaration has no word to call"),
    })
}

/// The parameters' values inside a body.
type Env = BTreeMap<String, Node>;

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
        self.value(body(decl)?, &free(decl))
    }

    /// A card's expansion.
    pub(crate) fn card(&self, decl: &Decl<'_>) -> anyhow::Result<Node> {
        let Shape::Value(value) = &decl.declaration.shape else {
            bail!("a card is a value");
        };
        self.value(value, &Env::new())
    }

    /// Each field of a declaration's block but its metadata, expanded with
    /// each parameter free.
    #[cfg(test)]
    pub(crate) fn fields(&self, decl: &Decl<'_>) -> anyhow::Result<Vec<(String, Node)>> {
        let env = free(decl);
        let Some(block) = decl.block() else {
            return Ok(Vec::new());
        };
        block
            .nodes()
            .filter(|field| !sylex_syl::METADATA_FIELDS.contains(&field.name.ident.as_str()))
            .map(|field| {
                let name = field.name.ident.as_str().to_owned();
                self.value(&field.value, &env)
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

    /// Expands `value`, whose parameters read `env`.
    pub(crate) fn value(&self, value: &Value, env: &Env) -> anyhow::Result<Node> {
        let node = match &value.kind {
            ValueKind::Parameter(name) => {
                let name = name.ident.as_str();
                env.get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("`${name}` reads no parameter"))?
            }
            ValueKind::String(text) => Node::String(text.clone()),
            ValueKind::Integer(number) => Node::Integer(*number),
            ValueKind::List(items) => Node::List(
                items
                    .iter()
                    .filter_map(Item::node)
                    .map(|item| self.value(item, env))
                    .collect::<anyhow::Result<_>>()?,
            ),
            ValueKind::Record(record) => {
                let mut fields: Vec<(String, Node)> = record
                    .nodes()
                    .map(|field| {
                        let node = self.value(&field.value, env)?;
                        Ok((field.name.ident.as_str().to_owned(), node))
                    })
                    .collect::<anyhow::Result<_>>()?;
                fields.sort_by(|a, b| a.0.cmp(&b.0));
                Node::Record(fields)
            }
            ValueKind::Call(call) => self.call(call, env)?,
        };
        let Some(index) = self.checked.resolutions.coerced(value) else {
            return Ok(node);
        };
        let coercion = &self.checked.decls[index];
        let [param] = coercion.params.as_slice() else {
            bail!("a coercion has one parameter");
        };
        self.value(body(coercion)?, &Env::from([(param.name.clone(), node)]))
    }

    fn call(&self, call: &Call, env: &Env) -> anyhow::Result<Node> {
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
                    let mut args = self.bind(call, decl, env)?;
                    Node::Call {
                        constructor: constructor.ident.as_str().to_owned(),
                        word: word.to_owned(),
                        sort: sort_name,
                        args: decl
                            .params
                            .iter()
                            .map(|param| args.remove(&param.name).expect("every parameter bound"))
                            .collect(),
                    }
                }
                Role::Template => self.value(body(decl)?, &self.bind(call, decl, env)?)?,
                Role::Definition => Node::Mark {
                    definition: word.to_owned(),
                    sort: sort_name,
                    deed: Self::deed(decl)?,
                    body: Box::new(self.value(body(decl)?, &self.bind(call, decl, env)?)?),
                },
            })
        };
        expand().with_context(|| format!("expanding `{word}` at `{sort}`"))
    }

    /// Each parameter of `decl` bound to its argument in `call`, expanded in
    /// the caller's `env`, or to its default, expanded where `decl` declares
    /// it, reading the parameters before it.
    fn bind(&self, call: &Call, decl: &Decl<'_>, env: &Env) -> anyhow::Result<Env> {
        let mut written: Vec<Option<&Value>> = vec![None; decl.params.len()];
        for arg in call.args.iter().filter_map(Item::node) {
            let index = self
                .checked
                .resolutions
                .arg(arg)
                .ok_or_else(|| anyhow!("an argument of `{}` binds nothing", call.head.ident))?;
            written[index] = Some(&arg.value);
        }
        let mut callee = Env::new();
        for (param, written) in header(decl)?
            .params
            .iter()
            .filter_map(Item::node)
            .zip(written)
        {
            let name = param.name.ident.as_str();
            let node = match (written, &param.default) {
                (Some(value), _) => self.value(value, env)?,
                (None, Some(default)) => self.value(default, &callee)?,
                (None, None) => bail!("`${name}` has no argument and no default"),
            };
            callee.insert(name.to_owned(), node);
        }
        Ok(callee)
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

/// Each of `decl`'s parameters, free.
fn free(decl: &Decl<'_>) -> Env {
    decl.params
        .iter()
        .map(|param| (param.name.clone(), Node::Param(param.name.clone())))
        .collect()
}
