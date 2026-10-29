//! Expected-type elaboration of bundle terms into the v3 syntax.
//!
//! Constructors use the signatures and defaults of the Lean syntax modules.
//! Bundle names are resolved by their result type before arguments are
//! elaborated, so an overloaded word never selects a declaration by filesystem
//! order.
mod compute;
mod schema;
mod snapshot;
mod types;

pub use snapshot::InvocationSamples;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use dpsi::Ident;
use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

use crate::bundle::Bundle;
use crate::bundle::BundleKey;
use crate::reader::Declaration;
use crate::reader::Token;
use crate::reader::{self};

/// The Lean syntax type expected at a Rust entry point.
pub trait SemanticType: DeserializeOwned {
    fn semantic_type() -> String;
}
impl<T: SemanticType> SemanticType for Box<T> {
    fn semantic_type() -> String {
        T::semantic_type()
    }
}
impl<T: SemanticType> SemanticType for Vec<T> {
    fn semantic_type() -> String {
        format!("List ({})", T::semantic_type())
    }
}
impl<T: SemanticType> SemanticType for Option<T> {
    fn semantic_type() -> String {
        format!("Option ({})", T::semantic_type())
    }
}
impl SemanticType for Ident {
    fn semantic_type() -> String {
        "String".into()
    }
}
impl SemanticType for u64 {
    fn semantic_type() -> String {
        "Nat".into()
    }
}
impl SemanticType for i64 {
    fn semantic_type() -> String {
        "Int".into()
    }
}
impl SemanticType for bool {
    fn semantic_type() -> String {
        "Bool".into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct LowerError {
    pub message: String,
}
impl LowerError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    fn at(self, context: impl std::fmt::Display) -> Self {
        Self::new(format!("{context}: {}", self.message))
    }
}
type Result<T> = std::result::Result<T, LowerError>;

#[derive(Clone)]
struct Field {
    name: Ident,
    rust_name: Ident,
    ty: String,
    default: Option<String>,
}
impl Field {
    fn new(name: &str, rust_name: &str, ty: &str, default: Option<&'static str>) -> Self {
        Self {
            name: Ident::new(name),
            rust_name: Ident::new(rust_name),
            ty: ty.into(),
            default: default.map(str::to_owned),
        }
    }
}
#[derive(Clone)]
struct Constructor {
    name: Ident,
    rust_name: Ident,
    fields: Vec<Field>,
}
impl Constructor {
    fn new(name: &str, rust_name: &str, fields: Vec<Field>) -> Self {
        Self {
            name: Ident::new(name),
            rust_name: Ident::new(rust_name),
            fields,
        }
    }
    fn value(&self, values: Vec<Value>) -> Value {
        let fields: Map<_, _> = self
            .fields
            .iter()
            .zip(values)
            .map(|(field, value)| (field.rust_name.to_string(), value))
            .collect();
        if self.rust_name.as_str().is_empty() {
            return Value::Object(fields);
        }
        if fields.is_empty() {
            return Value::String(self.rust_name.to_string());
        }
        Value::Object(Map::from_iter([(
            self.rust_name.to_string(),
            Value::Object(fields),
        )]))
    }
}
fn normal(ty: &str) -> String {
    let mut ty = ty.trim();
    while ty.starts_with('(') && ty.ends_with(')') {
        let mut depth = 0;
        let outer = ty.char_indices().all(|(i, c)| {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
            }
            depth != 0 || i + 1 == ty.len()
        });
        if !outer {
            break;
        }
        ty = ty[1..ty.len() - 1].trim();
    }
    let ty = ty.replace("Semantics.", "");
    for head in ["Option", "List", "Delta"] {
        if let Some(inner) = ty.strip_prefix(&format!("{head} ")) {
            let inner = normal(inner);
            return if inner.contains(' ') {
                format!("{head} ({inner})")
            } else {
                format!("{head} {inner}")
            };
        }
    }
    schema::alias(&ty).map_or(ty.clone(), normal)
}
fn shape(ty: &str) -> Option<Vec<Constructor>> {
    let ty = normal(ty);
    let (head, arg) = ty.split_once(' ').unwrap_or((&ty, ""));
    let mut shape = schema::constructors(head)?;
    for ctor in &mut shape {
        for field in &mut ctor.fields {
            field.ty = field.ty.replace('α', arg);
        }
    }
    Some(shape)
}
fn construct(ty: &str, name: &str, values: Vec<Value>) -> Result<Value> {
    let ctor = shape(ty)
        .and_then(|cs| cs.into_iter().find(|c| c.name.as_str() == name))
        .ok_or_else(|| LowerError::new(format!("unknown constructor {ty}.{name}")))?;
    if values.len() != ctor.fields.len() {
        return Err(LowerError::new(format!(
            "{ty}.{name} expects {} fields, got {}",
            ctor.fields.len(),
            values.len()
        )));
    }
    Ok(ctor.value(values))
}
fn is_atom(token: &Token, text: &str) -> bool {
    matches!(token, Token::Atom(a) if a == text)
}
fn split<'a>(tokens: &'a [Token], separator: &str) -> Vec<&'a [Token]> {
    tokens
        .split(|t| is_atom(t, separator))
        .filter(|t| !t.is_empty())
        .collect()
}
fn bare(name: &str) -> &str {
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || "_'.«»".contains(c))
    {
        return name;
    }
    name.rsplit('.')
        .next()
        .unwrap_or(name)
        .trim_matches(['«', '»'])
}

fn named_record_arguments(tokens: &[Token]) -> Result<Vec<Token>> {
    let fields = reader::fields(tokens).map_err(|e| LowerError::new(e.to_string()))?;
    Ok(fields
        .into_iter()
        .map(|(name, value)| {
            let mut field = vec![Token::Ident(name), Token::Atom(":=".into())];
            field.extend(value);
            Token::Group('(', field)
        })
        .collect())
}

fn nullary_word(name: &str, ty: &str) -> Option<Value> {
    if ["some", "none"].contains(&name) {
        return None;
    }
    let ty = ty.strip_prefix("Option ").unwrap_or(ty);
    let constructors = shape(ty)?;
    if !matches!(ty, "NounWord" | "ColorOrColorless" | "SimpleManaSymbol")
        && constructors.iter().any(|ctor| !ctor.fields.is_empty())
    {
        return None;
    }
    constructors
        .into_iter()
        .find(|ctor| ctor.name.as_str() == name && ctor.fields.is_empty())
        .map(|ctor| ctor.value(Vec::new()))
}

#[derive(Clone)]
struct Bound {
    ty: String,
    value: Value,
    identity: Option<Ident>,
    caller: bool,
}
#[derive(Clone)]
struct Function {
    input: String,
    output: String,
    tokens: Vec<Token>,
    scope: Box<Scope>,
}
#[derive(Clone, Default)]
struct Scope {
    bindings: BTreeMap<Ident, Bound>,
    seen: BTreeSet<Ident>,
    actor_identity: Option<Ident>,
    functions: BTreeMap<Ident, Function>,
    types: BTreeMap<Ident, String>,
}

/// A reader borrowing an already validated, ordered bundle.
pub struct Elaborator<'a> {
    bundle: &'a Bundle,
}
impl<'a> Elaborator<'a> {
    #[must_use]
    pub fn new(bundle: &'a Bundle) -> Self {
        Self { bundle }
    }

    /// Expand a term at its expected syntax type.
    ///
    /// # Errors
    /// Reports unknown names, invalid arguments, and ill-typed constructors.
    pub fn read<T: SemanticType>(&self, source: &str) -> Result<T> {
        let tokens = if T::semantic_type() == "Card"
            && self.bundle.items.contains_key(&BundleKey {
                bare_name: Ident::new(source),
                return_type: Ident::new("Spelled"),
            }) {
            vec![
                Token::Ident(Ident::new("cardNamed")),
                Token::String(source.to_owned()),
            ]
        } else {
            reader::tokenize(source).map_err(|e| LowerError::new(e.to_string()))?
        };
        let value = self.eval(
            &tokens,
            &T::semantic_type(),
            &mut Scope::default(),
            &mut Vec::new(),
        )?;
        serde_json::from_value(value)
            .map_err(|e| LowerError::new(format!("invalid {}: {e}", T::semantic_type())))
    }

    /// Read a registry value together with its conferrals.
    ///
    /// # Errors
    /// Rejects unknown keys, non-registry declarations, and invalid rule
    /// bodies.
    pub fn registry_definition(&self, key: &BundleKey) -> Result<crate::ast::RegistryDefinition> {
        let item = self
            .bundle
            .items
            .get(key)
            .ok_or_else(|| LowerError::new("unknown declaration key"))?;
        let decl = &item.declaration;
        let ctor = match decl.command.as_str() {
            "counter" => "counter",
            "subtype" => "subtype",
            "designation" => "designation",
            "card_type" => "cardType",
            "relation" => "relation",
            _ => return Err(LowerError::new("not a registry declaration")),
        };
        let mut scope = Scope::default();
        let mut stack = Vec::new();
        let mut fields = vec![self.registry(decl, &mut scope, &mut stack)?];
        if ctor != "relation" {
            let rules = decl
                .fields
                .get(&Ident::new("rules"))
                .or_else(|| decl.fields.get(&Ident::new("confers")));
            fields.push(if let Some(tokens) = rules {
                self.eval(tokens, "List Conferral", &mut scope, &mut stack)?
            } else {
                Value::Array(Vec::new())
            });
        }
        serde_json::from_value(construct("RegistryDefinition", ctor, fields)?)
            .map_err(|e| LowerError::new(e.to_string()))
    }

    fn card_body(
        &self,
        decl: &Declaration,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let tokens = decl
            .card
            .as_ref()
            .ok_or_else(|| LowerError::new("card requires an anonymous declaration"))?;
        self.eval(tokens, "Card", scope, stack)
    }

    fn declaration(&self, name: &str, ty: &str) -> Option<&Declaration> {
        if let Some(item) = self.bundle.items.get(&BundleKey {
            bare_name: Ident::new(name),
            return_type: Ident::new(if ty == "Card" { "Spelled" } else { ty }),
        }) {
            return Some(&item.declaration);
        }
        let ty = if ty == "Card"
            && self.bundle.items.contains_key(&BundleKey {
                bare_name: Ident::new(bare(name)),
                return_type: Ident::new("Spelled"),
            }) {
            "Spelled"
        } else {
            ty
        };
        let name = bare(name);
        let alias;
        let name = if name == "this" {
            "this_"
        } else if ty == "Predicate"
            && [
                "creature",
                "artifact",
                "enchantment",
                "land",
                "planeswalker",
                "battle",
                "instant",
                "sorcery",
                "kindred",
                "card",
                "permanent",
                "player",
                "token",
                "spell",
            ]
            .contains(&name)
        {
            let mut chars = name.chars();
            alias = format!("is{}{}", chars.next()?.to_uppercase(), chars.as_str());
            &alias
        } else {
            name
        };
        self.bundle
            .items
            .get(&BundleKey {
                bare_name: Ident::new(name),
                return_type: Ident::new(ty),
            })
            .map(|item| &item.declaration)
            .or_else(|| {
                self.bundle
                    .items
                    .values()
                    .map(|item| &item.declaration)
                    .find(|decl| {
                        decl.name.as_str() == name
                            && normal(decl.return_type.as_str()) == normal(ty)
                    })
            })
            .or_else(|| {
                self.bundle
                    .items
                    .values()
                    .map(|item| &item.declaration)
                    .find(|decl| {
                        decl.name.as_str() == name
                            && decl
                                .parameters
                                .iter()
                                .any(|p| p.implicit && p.name == decl.return_type)
                    })
            })
    }

    fn eval(
        &self,
        tokens: &[Token],
        expected: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let mut expected = expected.to_owned();
        for (name, ty) in &scope.types {
            expected = expected.replace(name.as_str(), &format!("({ty})"));
        }
        let expected = normal(&expected);
        self.eval_inner(tokens, &expected, scope, stack)
            .map_err(|e| e.at(format!("{} as {expected}", reader::render(tokens))))
    }
    fn bound_value(name: &str, bound: Bound, ty: &str, scope: &mut Scope) -> Result<Value> {
        if Self::coerce(bound.value.clone(), &bound.ty, ty).is_err()
            && let Some(word) = nullary_word(name, ty)
        {
            return Ok(word);
        }
        let mut value = bound.value;
        if let Some(identity) = bound.identity {
            let mention = if scope.seen.insert(identity) { "firstMention" } else { "laterMention" };
            if bound.caller {
                value = construct("NounPhrase", "inCaller", vec![value])?;
            }
            value = construct(
                "NounPhrase",
                mention,
                vec![Value::String(identity.to_string()), value],
            )?;
        }
        Self::coerce(value, &bound.ty, ty)
    }
    fn eval_inner(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        if tokens.is_empty() {
            return Err(LowerError::new("missing term"));
        }
        if matches!(ty, "Card" | "Spelled")
            && let [Token::Ident(command), Token::String(name)] = tokens
            && command.as_str() == "cardNamed"
        {
            let decl = self
                .bundle
                .items
                .get(&BundleKey {
                    bare_name: Ident::new(name),
                    return_type: Ident::new("Spelled"),
                })
                .ok_or_else(|| LowerError::new(format!("unknown card {name}")))?;
            return self.invoke(&decl.declaration, &[], "Card", scope, stack);
        }
        if let [Token::Group('(', body)] = tokens {
            return self.eval(body, ty, scope, stack);
        }
        if let Some(value) = self.compute(tokens, ty, scope, stack)? {
            return Ok(value);
        }
        if let Some(i) = tokens.iter().position(|t| is_atom(t, ":")) {
            let annotated = normal(&reader::render(&tokens[i + 1..]));
            if annotated != normal(ty) {
                return Err(LowerError::new(format!(
                    "annotation {annotated} does not match {ty}"
                )));
            }
            return self.eval(&tokens[..i], ty, scope, stack);
        }
        if let [Token::Ident(name)] = tokens
            && let Some(bound) = scope.bindings.get(name).cloned()
        {
            return Self::bound_value(name.as_str(), bound, ty, scope);
        }
        if ty.starts_with("Option ")
            && let [Token::Ident(name), rest @ ..] = tokens
            && let Some(decl) = self.declaration(name.as_str(), ty)
        {
            return self.invoke(decl, rest, ty, scope, stack);
        }
        if let Some(inner) = ty.strip_prefix("Option ") {
            return self.option_value(tokens, inner, scope, stack);
        }
        if let [Token::Group('[', elements)] = tokens {
            if elements.first().is_some_and(|t| is_atom(t, ","))
                || elements
                    .windows(2)
                    .any(|pair| pair.iter().all(|t| is_atom(t, ",")))
            {
                return Err(LowerError::new("missing list element"));
            }
            if let Some(inner) = ty.strip_prefix("List ") {
                return split(elements, ",")
                    .into_iter()
                    .map(|t| self.eval(t, inner, scope, stack))
                    .collect();
            }
            let (constructor, element_type) = match ty {
                "Action" => ("sequentially", "Action"),
                "Predicate" | "Condition" => ("and", ty),
                "StaticSpec" => ("conjunction", "StaticSpec"),
                "Cost" => {
                    let mut trial = scope.clone();
                    if let Ok(mana) = self.eval(tokens, "List ManaSymbol", &mut trial, stack) {
                        *scope = trial;
                        return construct("Cost", "mana", vec![mana]);
                    }
                    ("compound", "Cost")
                }
                _ => return Err(LowerError::new(format!("list literal is not a {ty}"))),
            };
            let values = self.eval(tokens, &format!("List {element_type}"), scope, stack)?;
            return if ty == "Predicate" {
                construct(
                    "Predicate",
                    "coord",
                    vec![construct("Coordinator", "and", vec![])?, values],
                )
            } else if ty == "StaticSpec" {
                construct(ty, constructor, vec![Value::Null, values])
            } else {
                construct(ty, constructor, vec![values])
            };
        }
        if let [Token::Group('{', fields)] = tokens {
            return self.record(fields, ty, scope, stack);
        }
        if let [Token::String(s)] = tokens {
            if ty == "String" {
                return Ok(Value::String(s.clone()));
            }
            if ty == "NameSource" {
                return construct(ty, "printed", vec![Value::String(s.clone())]);
            }
        }
        if matches!(tokens.first(), Some(t) if is_atom(t,"↑")) {
            return self.eval(&tokens[1..], ty, scope, stack);
        }
        let numeric = reader::render(tokens).replace(' ', "");
        if let Ok(n) = numeric.parse::<i64>() {
            match ty {
                "Nat" if n < 0 => return Err(LowerError::new("negative natural number")),
                "Nat" | "Int" => return Ok(Value::from(n)),
                "Amount" => return construct(ty, "lit", vec![Value::from(n)]),
                "ManaSymbol" if n >= 0 => {
                    return construct(
                        ty,
                        "simple",
                        vec![construct(
                            "SimpleManaSymbol",
                            "generic",
                            vec![Value::from(n)],
                        )?],
                    );
                }
                _ => {}
            }
        }
        self.application(tokens, ty, scope, stack)
    }

    fn application(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let (name, rest) = match tokens {
            [Token::Atom(dot), Token::Ident(name), rest @ ..] if dot == "." => {
                (name.as_str(), rest)
            }
            [Token::Ident(name), rest @ ..] => (name.as_str(), rest),
            _ => {
                return Err(LowerError::new(
                    "expected a literal, constructor, or declaration",
                ));
            }
        };
        if ty == "Deed"
            && rest.is_empty()
            && let Some(decl) =
                self.declaration(name.strip_suffix(".deed").unwrap_or(name), "Action")
            && decl.command.as_str() == "action"
        {
            let deed = self.deed(decl, scope, stack)?;
            if deed.is_null() {
                return Err(LowerError::new("definition does not name a deed"));
            }
            return Ok(deed);
        }
        if let Some(function) = scope.functions.get(&Ident::new(name)).cloned() {
            let argument = self.eval(rest, &function.input, scope, stack)?;
            let value = self.apply(
                &function.tokens,
                Bound {
                    ty: function.input,
                    value: argument,
                    identity: None,
                    caller: false,
                },
                &function.output,
                &mut function.scope.clone(),
                stack,
            )?;
            return Self::coerce(value, &function.output, ty);
        }
        if ty == "NounPhrase"
            && bare(name) == "theActor"
            && rest.is_empty()
            && let Some(identity) = scope.actor_identity
        {
            let mention = if scope.seen.insert(identity) { "firstMention" } else { "laterMention" };
            return construct(
                "NounPhrase",
                mention,
                vec![
                    Value::String(identity.to_string()),
                    construct("NounPhrase", "actor", vec![])?,
                ],
            );
        }
        if ty == "NounPhrase"
            && bare(name) == "inCaller"
            && let [Token::Ident(parameter)] = rest
            && let Some(bound) = scope.bindings.get(parameter)
            && normal(&bound.ty) == "NounPhrase"
        {
            return construct("NounPhrase", "inCaller", vec![bound.value.clone()]);
        }
        if ty == "Predicate"
            && bare(name) == "or"
            && let [Token::Group('[', alternatives)] = rest
        {
            let mut values = Vec::new();
            for alternative in split(alternatives, ",") {
                values.push(self.eval(alternative, ty, &mut scope.clone(), stack)?);
            }
            return construct(
                "Predicate",
                "coord",
                vec![
                    construct("Coordinator", "or", vec![])?,
                    Value::Array(values),
                ],
            );
        }
        if ty == "Bool" && rest.is_empty() && ["true", "false"].contains(&bare(name)) {
            return Ok(Value::Bool(bare(name) == "true"));
        }
        if name == "any" && matches!(tokens.first(), Some(Token::Ident(_))) && rest.is_empty() {
            return any_value(ty);
        }
        let raw = matches!(tokens.first(), Some(t) if is_atom(t, "."))
            || (name.starts_with("Semantics.")
                && !name.starts_with("Semantics.Templates.")
                && !name.starts_with("Semantics.Definitions."));
        if !raw && let Some(decl) = self.declaration(name, ty) {
            return self.invoke(decl, rest, ty, scope, stack);
        }
        if (raw || !["some", "none"].contains(&bare(name)))
            && let Some(ctor) =
                shape(ty).and_then(|cs| cs.into_iter().find(|c| c.name.as_str() == bare(name)))
        {
            // An explicit type qualifier must agree with the expected type.
            if let Some(qualifier) = name.rsplit('.').nth(1)
                && let base = ty.split_whitespace().next().unwrap_or(ty)
                && qualifier != base
            {
                return Err(LowerError::new(format!(
                    "constructor belongs to {qualifier}, expected {base}"
                )));
            }
            let named;
            let rest = if ty == "Card"
                && ctor.name.as_str() != "singleFaced"
                && let [Token::Group('{', fields)] = rest
            {
                named = named_record_arguments(fields)?;
                named.as_slice()
            } else {
                rest
            };
            let values = self.arguments(&ctor.fields, rest, scope, stack)?;
            return Ok(ctor.value(values));
        }
        if raw && let Some(decl) = self.declaration(name, ty) {
            return self.invoke(decl, rest, ty, scope, stack);
        }
        if ["KeywordLabel", "KeywordTerm"].contains(&ty)
            && rest.is_empty()
            && let Some(decl) =
                self.declaration(name.strip_suffix(".label").unwrap_or(name), "Ability")
            && decl.command.as_str() == "keyword"
        {
            let label = if let Some(tokens) = decl.fields.get(&Ident::new("label")) {
                self.eval(tokens, "String", scope, stack)?
            } else {
                Value::String(decl.name.to_string())
            };
            let label = construct("KeywordLabel", "named", vec![label])?;
            return if ty == "KeywordTerm" {
                construct("KeywordTerm", "the", vec![label])
            } else {
                Ok(label)
            };
        }
        // Only the coercions installed by Semantics.Primitives participate in
        // lookup.
        for source in coercion_sources(ty) {
            let mut trial = scope.clone();
            if let Ok(value) = self.eval(tokens, source, &mut trial, stack)
                && let Ok(coerced) = Self::coerce(value, source, ty)
            {
                *scope = trial;
                return Ok(coerced);
            }
        }
        Err(LowerError::new(format!("unknown {ty} term {name}")))
    }

    fn coerce(value: Value, from: &str, to: &str) -> Result<Value> {
        let from = normal(from);
        let to = normal(to);
        if from == to || (from == "Nat" && to == "Int") {
            return Ok(value);
        }
        if let Some(inner) = to.strip_prefix("Option ") {
            return Self::coerce(value, &from, inner);
        }
        let ctor = match (from.as_str(), to.as_str()) {
            ("NounPhrase", "Subject") => "actor",
            ("Subtype", "Predicate") => "hasSubtype",
            ("Action", "Cost") => "perform",
            ("List ManaSymbol", "Cost") => "mana",
            ("CharacteristicBundle", "TokenSpec") => "written",
            ("CounterKind", "CounterKindSource") | ("String", "NameSource") => "printed",
            ("Condition", "AsThough") => "holds",
            ("StaticSpec", "Ability") => "static",
            ("Color", "ColorOrColorless") => "of",
            ("Color", "ManaSymbol") => {
                return construct(
                    "ManaSymbol",
                    "simple",
                    vec![construct(
                        "SimpleManaSymbol",
                        "specific",
                        vec![construct("ColorOrColorless", "of", vec![value])?],
                    )?],
                );
            }
            ("Duration", "LookbackSpan") => {
                return construct("LookbackSpan", "duration", vec![value, Value::Null]);
            }
            ("Action", "Mode") => {
                return construct("Mode", "mk", vec![Value::Null, Value::Null, value]);
            }
            _ => return Err(LowerError::new(format!("no coercion from {from} to {to}"))),
        };
        construct(&to, ctor, vec![value])
    }

    fn arguments(
        &self,
        fields: &[Field],
        tokens: &[Token],
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Vec<Value>> {
        Self::argument_terms(fields, tokens)?
            .iter()
            .zip(fields)
            .map(|(term, field)| self.eval(term, &field.ty, scope, stack))
            .collect()
    }
    fn argument_terms(fields: &[Field], tokens: &[Token]) -> Result<Vec<Vec<Token>>> {
        let mut positional = Vec::new();
        let mut named = BTreeMap::new();
        let mut i = 0;
        while i < tokens.len() {
            if is_atom(&tokens[i], "<|") {
                positional.push(&tokens[i + 1..]);
                break;
            }
            if let Token::Group('(', content) = &tokens[i]
                && let [Token::Ident(name), Token::Atom(assign), rest @ ..] = content.as_slice()
                && assign == ":="
            {
                if named.insert(*name, rest).is_some() {
                    return Err(LowerError::new(format!("duplicate argument {name}")));
                }
                i += 1;
                continue;
            }
            let len =
                if is_atom(&tokens[i], ".") || is_atom(&tokens[i], "↑") || is_atom(&tokens[i], "-")
                {
                    2
                } else {
                    1
                };
            if i + len > tokens.len() {
                return Err(LowerError::new("incomplete argument"));
            }
            positional.push(&tokens[i..i + len]);
            i += len;
        }
        let mut positional = positional.into_iter();
        let mut values = Vec::new();
        for field in fields {
            let term = if let Some(term) = named.remove(&field.name).or_else(|| positional.next()) {
                term.to_vec()
            } else if let Some(default) = &field.default {
                reader::tokenize(default).map_err(|e| LowerError::new(e.to_string()))?
            } else {
                return Err(LowerError::new(format!(
                    "missing argument {} : {}",
                    field.name, field.ty
                )));
            };
            values.push(term);
        }
        if positional.next().is_some() || !named.is_empty() {
            return Err(LowerError::new("unexpected arguments"));
        }
        Ok(values)
    }
    fn option_value(
        &self,
        tokens: &[Token],
        inner: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let option_tokens = if tokens.first().is_some_and(|t| is_atom(t, ".")) {
            &tokens[1..]
        } else {
            tokens
        };
        if matches!(option_tokens, [Token::Ident(n)] if bare(n.as_str()) == "none") {
            return Ok(Value::Null);
        }
        let value = if matches!(option_tokens.first(), Some(Token::Ident(n)) if bare(n.as_str()) == "some")
        {
            let value = &option_tokens[1..];
            if value.first().is_some_and(|token| is_atom(token, "<|")) {
                &value[1..]
            } else {
                value
            }
        } else {
            tokens
        };
        self.eval(value, inner, scope, stack)
    }

    fn record(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        if ty == "CharacteristicBundle" {
            let fields = reader::fields(tokens).map_err(|e| LowerError::new(e.to_string()))?;
            if !fields.contains_key(&Ident::new("characteristics"))
                && !fields.contains_key(&Ident::new("qualities"))
            {
                let characteristics = self.record(tokens, "Characteristics", scope, stack)?;
                return construct(ty, "mk", vec![characteristics, Value::Array(Vec::new())]);
            }
        }
        if ty == "Card"
            || (ty == "CardFace"
                && !reader::fields(tokens)
                    .map_err(|e| LowerError::new(e.to_string()))?
                    .contains_key(&Ident::new("characteristics")))
        {
            let characteristics = self.record(tokens, "Characteristics", scope, stack)?;
            let face = construct("CardFace", "mk", vec![characteristics])?;
            return if ty == "Card" {
                construct("Card", "singleFaced", vec![face])
            } else {
                Ok(face)
            };
        }
        let cs = shape(ty).ok_or_else(|| LowerError::new(format!("{ty} is not a structure")))?;
        let ctor = cs
            .into_iter()
            .find(|c| c.rust_name.as_str().is_empty())
            .ok_or_else(|| LowerError::new(format!("{ty} is not a structure")))?;
        let arguments = split(tokens, ",")
            .into_iter()
            .map(|ts| {
                if let [Token::Ident(name)] = ts {
                    Token::Group(
                        '(',
                        vec![
                            Token::Ident(*name),
                            Token::Atom(":=".into()),
                            Token::Ident(*name),
                        ],
                    )
                } else {
                    Token::Group('(', ts.to_vec())
                }
            })
            .collect::<Vec<_>>();
        Ok(ctor.value(self.arguments(&ctor.fields, &arguments, scope, stack)?))
    }

    fn invoke(
        &self,
        decl: &Declaration,
        tokens: &[Token],
        expected: &str,
        caller: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let key = BundleKey {
            bare_name: decl.name,
            return_type: decl.return_type,
        };
        if stack.contains(&key) {
            if stack.last() == Some(&key)
                && tokens.is_empty()
                && decl.command.as_str() == "template"
                && let Some(word) = nullary_word(decl.name.as_str(), expected)
            {
                return Ok(word);
            }
            return Err(LowerError::new(format!(
                "recursive expansion of {} : {}",
                decl.name, decl.return_type
            )));
        }
        let fields = decl
            .parameters
            .iter()
            .filter(|p| !p.implicit)
            .map(|p| {
                let (ty, default) = if matches!(p.ty.first(), Some(Token::Ident(n)) if n.as_str() == "optParam") && p.ty.len() >= 3 {
                    (reader::render(&p.ty[1..2]), Some(reader::render(&p.ty[2..])))
                } else { (reader::render(&p.ty), p.default.as_deref().map(reader::render)) };
                Field { name: p.name, rust_name: p.name, ty, default }
            })
            .collect::<Vec<_>>();
        let mut local = Scope::default();
        let mut arguments = Vec::new();
        for token in tokens {
            if let Token::Group('(', ts) = token
                && let [Token::Ident(name), Token::Atom(assign), value @ ..] = ts.as_slice()
                && assign == ":="
                && decl
                    .parameters
                    .iter()
                    .any(|p| p.implicit && p.name == *name)
            {
                local.types.insert(*name, normal(&reader::render(value)));
                continue;
            }
            arguments.push(token.clone());
        }
        for parameter in decl.parameters.iter().filter(|p| p.implicit) {
            if decl.return_type == parameter.name {
                local.types.insert(parameter.name, expected.to_owned());
            }
        }
        let terms = Self::argument_terms(&fields, &arguments)?;
        for (field, term) in fields.iter().zip(&terms) {
            for parameter in decl.parameters.iter().filter(|p| p.implicit) {
                if field.ty.contains(parameter.name.as_str())
                    && !local.types.contains_key(&parameter.name)
                    && !field.ty.contains('→')
                {
                    let (actual, _) = self.infer(term, caller, stack)?;
                    let ty = if normal(&field.ty).starts_with("List ") {
                        normal(&actual)
                            .strip_prefix("List ")
                            .map(str::to_owned)
                            .ok_or_else(|| LowerError::new("expected generic list argument"))?
                    } else {
                        actual
                    };
                    local.types.insert(parameter.name, normal(&ty));
                }
            }
        }
        let body = decl.fields.get(&Ident::new("body"));
        let definition = ["action", "keyword"].contains(&decl.command.as_str());
        for (field, term) in fields.iter().zip(&terms) {
            let mut ty = field.ty.clone();
            for (name, replacement) in &local.types {
                ty = ty.replace(name.as_str(), &format!("({replacement})"));
            }
            let ty = normal(&ty);
            if let Some((input, output)) = ty.split_once('→').or_else(|| ty.split_once("->")) {
                local.functions.insert(
                    field.name,
                    Function {
                        input: normal(input),
                        output: normal(output),
                        tokens: term.clone(),
                        scope: Box::new(caller.clone()),
                    },
                );
                continue;
            }
            let mut value = self.eval(term, &ty, caller, stack)?;
            let repeated = body.is_some_and(|ts| count_name(ts, field.name) >= 2);
            let identity = if ty == "NounPhrase"
                && (definition || (decl.command.as_str() == "template" && repeated))
            {
                Some(Ident::new(&format!("{}.{}", decl.name, field.name)))
            } else {
                None
            };
            if definition && ty != "NounPhrase" {
                value = in_caller(value, &ty)?;
            }
            local.bindings.insert(
                field.name,
                Bound {
                    ty,
                    value,
                    identity,
                    caller: definition,
                },
            );
        }
        if decl.command.as_str() == "action"
            && body.is_some_and(|ts| count_bare_name(ts, "theActor") >= 2)
        {
            local.actor_identity = Some(Ident::new(&format!("{}.actor", decl.name)));
        }
        stack.push(key);
        let result = self.declaration_body(decl, &mut local, stack);
        stack.pop();
        result.map_err(|e| e.at(format!("declaration {} : {}", decl.name, decl.return_type)))
    }
    fn deed(
        &self,
        decl: &Declaration,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let written = decl.fields.get(&Ident::new("deed"));
        if let Some(ability) = decl.fields.get(&Ident::new("ofAbility")) {
            if written.is_some() || decl.fields.contains_key(&Ident::new("permanent")) {
                return Err(LowerError::new(
                    "ofAbility cannot be combined with deed or permanent",
                ));
            }
            return construct(
                "Deed",
                "ofAbility",
                vec![self.eval(ability, "KeywordLabel", scope, stack)?],
            );
        }
        if matches!(written.map(Vec::as_slice), Some([Token::Ident(n)]) if n.as_str() == "none") {
            return Ok(Value::Null);
        }
        let label = match written.map(Vec::as_slice) {
            Some([Token::Ident(n)]) => Value::String(n.to_string()),
            Some(tokens) => self.eval(tokens, "String", scope, stack)?,
            None => Value::String(decl.name.to_string()),
        };
        let permanent = if let Some(tokens) = decl.fields.get(&Ident::new("permanent")) {
            self.eval(tokens, "DeedPermanent", scope, stack)?
        } else {
            construct("DeedPermanent", "none", vec![])?
        };
        construct("Deed", "action", vec![label, permanent])
    }

    fn declaration_body(
        &self,
        decl: &Declaration,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let field = |name: &str,
                     ty: &str,
                     default: Option<&str>,
                     scope: &mut Scope,
                     stack: &mut Vec<BundleKey>|
         -> Result<Value> {
            if let Some(tokens) = decl.fields.get(&Ident::new(name)) {
                self.eval(tokens, ty, scope, stack)
            } else if let Some(term) = default {
                self.eval(
                    &reader::tokenize(term).map_err(|e| LowerError::new(e.to_string()))?,
                    ty,
                    scope,
                    stack,
                )
            } else {
                Err(LowerError::new(format!("{} requires {name}", decl.command)))
            }
        };
        match decl.command.as_str() {
            "card" => self.card_body(decl, scope, stack),
            "primitive" | "template" => {
                field("body", decl.return_type.as_str(), None, scope, stack)
            }
            "action" => {
                let deed = self.deed(decl, scope, stack)?;
                construct(
                    "Action",
                    "ownScope",
                    vec![deed, field("body", "Action", None, scope, stack)?],
                )
            }
            "keyword" => {
                let mut params = Vec::new();
                for p in &decl.parameters {
                    if p.implicit {
                        continue;
                    }
                    let bound = &scope.bindings[&p.name];
                    let ctor = match bound.ty.as_str() {
                        "NounPhrase" | "Action" => continue,
                        "Cost" => "cost",
                        "Amount" => "number",
                        "Predicate" if p.name.as_str() == "subject" => "subject",
                        "Predicate" => "quality",
                        "DeckCondition" => "deckCondition",
                        "Ability" => "ability",
                        ty => return Err(LowerError::new(format!("no KeywordParam for {ty}"))),
                    };
                    params.push(construct("KeywordParam", ctor, vec![bound.value.clone()])?);
                }
                construct(
                    "Ability",
                    "keyword",
                    vec![
                        construct(
                            "KeywordLabel",
                            "named",
                            vec![field(
                                "label",
                                "String",
                                Some(&format!("{:?}", decl.name.as_str())),
                                scope,
                                stack,
                            )?],
                        )?,
                        Value::Array(params),
                        field("body", "List Ability", Some("[]"), scope, stack)?,
                    ],
                )
            }
            "ability_word" => {
                let body = field("body", "Ability", None, scope, stack)?;
                if body
                    .get("ItalicHead")
                    .and_then(|h| h.get("word"))
                    .and_then(|w| w.get("AbilityWord"))
                    .is_some()
                {
                    return Ok(body);
                }
                construct(
                    "Ability",
                    "italicHead",
                    vec![
                        construct(
                            "ItalicWord",
                            "abilityWord",
                            vec![construct(
                                "AbilityWordLabel",
                                "named",
                                vec![field(
                                    "label",
                                    "String",
                                    Some(&format!("{:?}", decl.name.as_str())),
                                    scope,
                                    stack,
                                )?],
                            )?],
                        )?,
                        body,
                    ],
                )
            }
            "counter" | "relation" | "designation" | "subtype" | "card_type" => {
                self.registry(decl, scope, stack)
            }
            _ => {
                if decl.fields.contains_key(&Ident::new("body")) {
                    field("body", decl.return_type.as_str(), None, scope, stack)
                } else {
                    let mut fields = Vec::new();
                    for (name, value) in &decl.fields {
                        if ["lexical", "spelling"].contains(&name.as_str()) {
                            continue;
                        }
                        if !fields.is_empty() {
                            fields.push(Token::Atom(",".into()));
                        }
                        fields.extend([Token::Ident(*name), Token::Atom(":=".into())]);
                        fields.extend(value.clone());
                    }
                    self.record(&fields, decl.return_type.as_str(), scope, stack)
                }
            }
        }
    }
    fn registry(
        &self,
        decl: &Declaration,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let field = |name: &str,
                     ty: &str,
                     default: Option<&str>,
                     scope: &mut Scope,
                     stack: &mut Vec<BundleKey>|
         -> Result<Value> {
            if let Some(tokens) = decl.fields.get(&Ident::new(name)) {
                self.eval(tokens, ty, scope, stack)
            } else if let Some(term) = default {
                self.eval(
                    &reader::tokenize(term).map_err(|e| LowerError::new(e.to_string()))?,
                    ty,
                    scope,
                    stack,
                )
            } else {
                Err(LowerError::new(format!("{} requires {name}", decl.command)))
            }
        };
        let label = Value::String(decl.name.to_string());
        match decl.command.as_str() {
            "counter" => construct(
                "CounterKind",
                "named",
                vec![
                    label,
                    field("holder", "Kind", Some("object"), scope, stack)?,
                ],
            ),
            "relation" => construct("RelationLabel", "named", vec![label]),
            "designation" => construct(
                "DesignationLabel",
                "named",
                vec![
                    label,
                    field("scope", "DesignationScope", None, scope, stack)?,
                    field("sole", "Bool", Some("false"), scope, stack)?,
                ],
            ),
            "subtype" => {
                let name = field(
                    "name",
                    "String",
                    Some(&format!("{:?}", decl.name.as_str())),
                    scope,
                    stack,
                )?;
                if decl.fields.get(&Ident::new("kind")).is_some_and(|ts| {
                    reader::render(ts).replace(' ', "").trim_start_matches('.') == "spell"
                }) {
                    construct("Subtype", "spell", vec![name])
                } else {
                    construct(
                        "Subtype",
                        "of",
                        vec![field("kind", "CardType", None, scope, stack)?, name],
                    )
                }
            }
            "card_type" => Ok(Value::Object(Map::from_iter([
                (
                    "name".into(),
                    field(
                        "name",
                        "String",
                        Some(&format!("{:?}", decl.name.as_str())),
                        scope,
                        stack,
                    )?,
                ),
                (
                    "card_class".into(),
                    field("cardClass", "Option CardClass", Some("none"), scope, stack)?,
                ),
                (
                    "stats".into(),
                    field("stats", "List Stat", Some("[]"), scope, stack)?,
                ),
            ]))),
            _ => Err(LowerError::new("not a registry declaration")),
        }
    }
}
fn coercion_sources(ty: &str) -> &'static [&'static str] {
    match ty {
        "Subject" => &["NounPhrase"],
        "Predicate" => &["Subtype"],
        "Cost" | "Mode" => &["Action"],
        "TokenSpec" => &["CharacteristicBundle"],
        "CounterKindSource" => &["CounterKind"],
        "NameSource" => &["String"],
        "AsThough" => &["Condition"],
        "ManaSymbol" | "ColorOrColorless" => &["Color"],
        "Ability" => &["StaticSpec"],
        "LookbackSpan" => &["Duration"],
        _ => &[],
    }
}
fn count_name(tokens: &[Token], name: Ident) -> usize {
    if matches!(tokens.first(), Some(Token::Ident(word)) if word.as_str() == "match")
        && let Some(with) = tokens
            .iter()
            .position(|t| matches!(t, Token::Ident(word) if word.as_str() == "with"))
    {
        let branches = split(&tokens[with + 1..], "|");
        let branch_count = branches
            .into_iter()
            .filter_map(|branch| {
                branch
                    .iter()
                    .position(|t| is_atom(t, "=>"))
                    .map(|arrow| count_name(&branch[arrow + 1..], name))
            })
            .max()
            .unwrap_or(0);
        return count_name(&tokens[1..with], name) + branch_count;
    }
    let direct = match tokens {
        [Token::Ident(constructor), Token::Ident(argument)] => Some((constructor, argument)),
        [
            Token::Atom(dot),
            Token::Ident(constructor),
            Token::Ident(argument),
        ] if dot == "." => Some((constructor, argument)),
        _ => None,
    };
    if direct.is_some_and(|(constructor, argument)| {
        bare(constructor.as_str()) == "inCaller" && *argument == name
    }) {
        return 0;
    }
    tokens
        .iter()
        .map(|token| match token {
            Token::Ident(n) => usize::from(*n == name),
            Token::Group(_, ts) => count_name(ts, name),
            _ => 0,
        })
        .sum()
}
fn in_caller(mut value: Value, ty: &str) -> Result<Value> {
    let ty = normal(ty);
    if ty == "PlayPayment" {
        for variant in ["PayingInstead", "PlusAdditional", "FaceDown", "Transformed"] {
            if let Some(cost) = value.get_mut(variant).and_then(|v| v.get_mut("cost")) {
                *cost = construct("Cost", "inCaller", vec![cost.take()])?;
                break;
            }
        }
        return Ok(value);
    }
    if ty == "Ballot" {
        if let Some(candidates) = value
            .get_mut("ByCandidate")
            .and_then(|v| v.get_mut("candidates"))
        {
            *candidates = construct("NounPhrase", "inCaller", vec![candidates.take()])?;
        }
        return Ok(value);
    }
    if let Some(inner) = ty.strip_prefix("Option ") {
        return if value.is_null() { Ok(value) } else { in_caller(value, inner) };
    }
    if let Some(inner) = ty.strip_prefix("List ") {
        let Value::Array(values) = value else {
            return Err(LowerError::new("invalid list argument"));
        };
        return values.into_iter().map(|v| in_caller(v, inner)).collect();
    }
    if [
        "Action",
        "Cost",
        "StaticSpec",
        "Replacement",
        "Amount",
        "Predicate",
        "Quantity",
        "Location",
        "Condition",
        "NounPhrase",
        "GameEvent",
        "Ability",
        "TokenSpec",
        "MoveRider",
    ]
    .contains(&ty.as_str())
    {
        construct(&ty, "inCaller", vec![value])
    } else {
        Ok(value)
    }
}

fn count_bare_name(tokens: &[Token], name: &str) -> usize {
    tokens
        .iter()
        .map(|token| match token {
            Token::Ident(n) => usize::from(bare(n.as_str()) == name),
            Token::Group(_, ts) => count_bare_name(ts, name),
            _ => 0,
        })
        .sum()
}

fn any_value(ty: &str) -> Result<Value> {
    let ctor = match ty {
        "Subject" | "NounPhrase" => "any",
        "Ballot" => "anyBallot",
        "Cost" => "anyCost",
        "Amount" => "anyAmount",
        "CounterKindSource" => "anyKind",
        "RollWatch" => "anyResult",
        "Location" => "anyLocation",
        "Timing" => "anyTiming",
        "DieSides" => "anyDie",
        _ => return Err(LowerError::new(format!("no AnyValue instance for {ty}"))),
    };
    construct(ty, ctor, vec![])
}
