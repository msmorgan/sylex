//! Loose declaration bundles and dependency ordering matching `Authoring.lean`.
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use dpsi::Ident;

use crate::reader::Declaration;
use crate::reader::ReadError;
use crate::reader::Token;
use crate::reader::{self};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundleKey {
    pub bare_name: Ident,
    pub return_type: Ident,
}
#[derive(Clone, Debug)]
pub struct BundleItem {
    pub declaration: Declaration,
    pub file: PathBuf,
}
impl BundleItem {
    #[must_use]
    pub fn key(&self) -> BundleKey {
        BundleKey {
            bare_name: self.declaration.name,
            return_type: self.declaration.return_type,
        }
    }
    fn registered(&self) -> bool {
        matches!(
            self.declaration.command.as_str(),
            "counter" | "subtype" | "designation" | "card_type" | "relation"
        )
    }
    fn references(&self, include_bound: bool) -> BTreeSet<Ident> {
        fn visit(tokens: &[Token], bound: &BTreeSet<Ident>, references: &mut BTreeSet<Ident>) {
            for (index, token) in tokens.iter().enumerate() {
                match token {
                    Token::Ident(id)
                        if !bound.contains(id)
                            && !matches!(index.checked_sub(1).and_then(|i| tokens.get(i)), Some(Token::Atom(a)) if a == ".")
                            && !matches!(tokens.get(index + 1), Some(Token::Atom(a)) if a == ":=") =>
                    {
                        references.insert(*id);
                        let mut name = id.as_str();
                        for prefix in [
                            "Semantics.Templates.",
                            "Templates.",
                            "Semantics.Definitions.",
                        ] {
                            if let Some(bare) = name.strip_prefix(prefix) {
                                name = bare;
                                break;
                            }
                        }
                        references.insert(Ident::new(name));
                        if name == "this" {
                            references.insert(Ident::new("this_"));
                        }
                        if [
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
                            if let Some(first) = chars.next() {
                                references.insert(Ident::new(&format!(
                                    "is{}{rest}",
                                    first.to_uppercase(),
                                    rest = chars.as_str()
                                )));
                            }
                        }
                    }
                    Token::Group(_, inner) => visit(inner, bound, references),
                    _ => {}
                }
            }
        }
        if self.declaration.command.as_str() == "primitive" {
            return BTreeSet::new();
        }
        let bound = if include_bound {
            BTreeSet::new()
        } else {
            self.declaration.parameters.iter().map(|p| p.name).collect()
        };
        let mut refs = BTreeSet::new();
        for parameter in &self.declaration.parameters {
            visit(&parameter.ty, &bound, &mut refs);
            if let Some(default) = &parameter.default {
                visit(default, &bound, &mut refs);
            }
        }
        if let Some(card) = &self.declaration.card {
            visit(card, &bound, &mut refs);
        }
        for (key, value) in &self.declaration.fields {
            if !matches!(key.as_str(), "lexical" | "spelling" | "ofAbility") {
                visit(value, &bound, &mut refs);
            }
        }
        if self.declaration.command.as_str() == "ability_word" {
            refs.insert(Ident::new("abilityWord"));
        }
        refs
    }
}

#[derive(Clone, Debug)]
pub struct Bundle {
    pub items: BTreeMap<BundleKey, BundleItem>,
    pub order: Vec<BundleKey>,
}
impl Bundle {
    /// Load all `.lean` files recursively. Paths carry no semantic information.
    ///
    /// # Errors
    /// Reports IO, malformed declaration, duplicate key, and exact cycle
    /// errors.
    pub fn load(path: &Path) -> Result<Self, ReadError> {
        fn walk(path: &Path, items: &mut Vec<BundleItem>) -> Result<(), ReadError> {
            let entries = std::fs::read_dir(path)
                .map_err(|e| ReadError(format!("{}: {e}", path.display())))?;
            for entry in entries {
                let entry = entry.map_err(|e| ReadError(e.to_string()))?;
                let file = entry.path();
                let ty = entry.file_type().map_err(|e| ReadError(e.to_string()))?;
                if ty.is_dir() {
                    walk(&file, items)?;
                } else if ty.is_file() && file.extension().is_some_and(|e| e == "lean") {
                    let source = std::fs::read_to_string(&file)
                        .map_err(|e| ReadError(format!("{}: {e}", file.display())))?;
                    let declaration = reader::parse(&source)
                        .map_err(|e| ReadError(format!("{}: {e}", file.display())))?;
                    items.push(BundleItem { declaration, file });
                }
            }
            Ok(())
        }
        let mut items = Vec::new();
        walk(path, &mut items)?;
        Self::from_items(items)
    }

    /// Order declarations with the lexicographically least ready key first.
    ///
    /// # Errors
    /// Reports duplicate keys and only nodes participating in dependency
    /// cycles.
    pub fn from_items(items: Vec<BundleItem>) -> Result<Self, ReadError> {
        let mut by_key = BTreeMap::new();
        for item in items {
            let key = item.key();
            if by_key.insert(key.clone(), item).is_some() {
                return Err(ReadError(format!(
                    "duplicate declaration {} : {}",
                    key.bare_name, key.return_type
                )));
            }
        }
        let mut dependencies = BTreeMap::new();
        let mut dependents: BTreeMap<BundleKey, BTreeSet<BundleKey>> = BTreeMap::new();
        let mut degrees = BTreeMap::new();
        let mut ready = BTreeSet::new();
        for (key, item) in &by_key {
            let refs = item.references(false);
            // A non-callable parameter can shadow a primitive in application position.
            // Conservatively order primitive leaves before all matching references.
            let primitive_refs = item.references(true);
            let deps: BTreeSet<_> = by_key
                .iter()
                .filter_map(|(other, value)| {
                    let refs = if value.declaration.command.as_str() == "primitive" {
                        &primitive_refs
                    } else {
                        &refs
                    };
                    let named = if value.registered() {
                        refs.contains(&Ident::new(&format!("{}_definition", other.bare_name)))
                    } else {
                        refs.contains(&other.bare_name)
                            || refs.contains(&Ident::new(&format!(
                                "{}.{}",
                                other.return_type, other.bare_name
                            )))
                    };
                    (named
                        && other != key
                        && (other.bare_name != key.bare_name
                            || value.declaration.command.as_str() == "primitive"
                            || (key.return_type.as_str() == "Ability"
                                && other.return_type.as_str() == "Action")))
                        .then(|| other.clone())
                })
                .collect();
            degrees.insert(key.clone(), deps.len());
            if deps.is_empty() {
                ready.insert(key.clone());
            }
            for dep in &deps {
                dependents
                    .entry(dep.clone())
                    .or_default()
                    .insert(key.clone());
            }
            dependencies.insert(key.clone(), deps);
        }
        let mut order = Vec::new();
        while let Some(key) = ready.pop_first() {
            if let Some(users) = dependents.get(&key) {
                for user in users {
                    let degree = degrees
                        .get_mut(user)
                        .ok_or_else(|| ReadError("dependent has no degree".into()))?;
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(user.clone());
                    }
                }
            }
            order.push(key);
        }
        if order.len() != by_key.len() {
            let cyclic: Vec<_> = by_key
                .keys()
                .filter(|key| {
                    let mut pending: Vec<_> = dependencies[*key].iter().cloned().collect();
                    let mut seen = BTreeSet::new();
                    while let Some(next) = pending.pop() {
                        if next == **key {
                            return true;
                        }
                        if seen.insert(next.clone()) {
                            pending.extend(dependencies[&next].iter().cloned());
                        }
                    }
                    false
                })
                .map(|key| {
                    format!(
                        "{} : {} ({})",
                        key.bare_name,
                        key.return_type,
                        by_key[key].file.display()
                    )
                })
                .collect();
            return Err(ReadError(format!(
                "dependency cycle: {}",
                cyclic.join(", ")
            )));
        }
        Ok(Self {
            items: by_key,
            order,
        })
    }
}
