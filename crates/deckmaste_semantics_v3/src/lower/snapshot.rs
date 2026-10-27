//! Typed declaration snapshots for expansion comparisons.
use std::collections::BTreeSet;
use std::fmt::Debug;

use dpsi::Ident;

use super::BundleKey;
use super::Elaborator;
use super::LowerError;
use super::Result;
use super::Scope;
use super::Value;
use super::normal;
use super::reader;
use super::shape;
use super::types;
use crate::ast;

pub(super) fn decode<T: serde::de::DeserializeOwned + Debug + 'static>(
    value: Value,
) -> Result<Box<dyn Debug>> {
    serde_json::from_value::<T>(value)
        .map(|v| Box::new(v) as Box<dyn Debug>)
        .map_err(|e| LowerError::new(e.to_string()))
}
pub(super) fn composite(ty: &str, value: Value) -> Result<Box<dyn Debug>> {
    match ty {
        "Nat" => decode::<u64>(value),
        "Int" => decode::<i64>(value),
        "String" => decode::<Ident>(value),
        "Bool" => decode::<bool>(value),
        "QualityPayload" => decode::<super::characteristics::QualityPayload>(value),
        "Delta Amount" => decode::<ast::Delta<Box<ast::Amount>>>(value),
        "List CopyExcept" => decode::<Vec<ast::CopyExcept>>(value),
        "List Predicate" => decode::<Vec<ast::Predicate>>(value),
        "Option Amount" => decode::<Option<ast::Amount>>(value),
        _ => Err(LowerError::new(format!("no typed snapshot for {ty}"))),
    }
}
impl Elaborator<'_> {
    /// Expand a declaration and render its typed value, including registry
    /// rules. `result_type` supplies the expected result for a polymorphic
    /// declaration.
    ///
    /// # Errors
    /// Reports unknown declarations, missing result types, and invalid
    /// expansions.
    pub fn snapshot(
        &self,
        key: &BundleKey,
        arguments: &str,
        result_type: Option<Ident>,
    ) -> Result<String> {
        let item = self
            .bundle
            .items
            .get(key)
            .ok_or_else(|| LowerError::new("unknown declaration key"))?;
        let decl = &item.declaration;
        if ["counter", "designation", "subtype", "card_type", "relation"]
            .contains(&decl.command.as_str())
        {
            return Ok(format!("{:#?}", self.registry_definition(key)?));
        }
        let declared =
            if decl.command.as_str() == "card" { "Card" } else { decl.return_type.as_str() };
        let expected = result_type.map_or_else(|| normal(declared), |ty| normal(ty.as_str()));
        if expected != normal(declared)
            && !decl
                .parameters
                .iter()
                .any(|p| p.implicit && p.name.as_str() == declared)
        {
            return Err(LowerError::new(
                "snapshot result disagrees with the declaration's type",
            ));
        }
        let tokens = reader::tokenize(arguments).map_err(|e| LowerError::new(e.to_string()))?;
        let value = self.invoke(
            decl,
            &tokens,
            &expected,
            &mut Scope::default(),
            &mut Vec::new(),
        )?;
        Ok(format!("{:#?}", types::debug_value(&expected, value)?))
    }

    /// Deterministic constructor-basis arguments for inspecting a declaration.
    /// Defaulted parameters are omitted; polymorphic type parameters use `Nat`.
    ///
    /// # Errors
    /// Reports parameter types for which a finite sample cannot be constructed.
    pub fn sample_arguments(&self, key: &BundleKey) -> Result<(String, Option<Ident>)> {
        let decl = &self
            .bundle
            .items
            .get(key)
            .ok_or_else(|| LowerError::new("unknown declaration key"))?
            .declaration;
        let mut types = Vec::new();
        let mut result = None;
        let mut args = Vec::new();
        for p in decl.parameters.iter().filter(|p| p.implicit) {
            if reader::render(&p.ty) != "Type" {
                return Err(LowerError::new("unsupported implicit sample parameter"));
            }
            types.push(p.name);
            args.push(format!("({} := Nat)", p.name));
            if p.name == decl.return_type {
                result = Some(Ident::new("Nat"));
            }
        }
        for (index, p) in decl.parameters.iter().filter(|p| !p.implicit).enumerate() {
            if p.default.is_some()
                || matches!(p.ty.first(),Some(reader::Token::Ident(n)) if n.as_str()=="optParam")
            {
                continue;
            }
            let mut ty = reader::render(&p.ty);
            for name in &types {
                ty = ty.replace(name.as_str(), "Nat");
            }
            let value = sample(&ty, index, &mut BTreeSet::new())?;
            args.push(format!("({} := {value})", p.name));
        }
        Ok((args.join(" "), result))
    }
}
fn sample(ty: &str, ordinal: usize, visiting: &mut BTreeSet<String>) -> Result<String> {
    let ty = normal(ty);
    match ty.as_str() {
        "Nat" | "Int" | "Amount" => return Ok((ordinal + 3).to_string()),
        "String" => return Ok(format!("\"Sample {ordinal}\"")),
        "Bool" => return Ok("false".into()),
        "NounPhrase" => return Ok("you".into()),
        "Cost" => return Ok("[2]".into()),
        _ => {}
    }
    if ty.starts_with("Option ") {
        return Ok("none".into());
    }
    if ty.starts_with("List ") {
        return Ok("[]".into());
    }
    if let Some((input, output)) = ty.split_once('→').or_else(|| ty.split_once("->")) {
        return Ok(format!(
            "fun (sample : {input}) => {}",
            sample(output, ordinal, visiting)?
        ));
    }
    if !visiting.insert(ty.clone()) {
        return Err(LowerError::new(format!("recursive sample type {ty}")));
    }
    let mut constructors =
        shape(&ty).ok_or_else(|| LowerError::new(format!("unknown sample type {ty}")))?;
    constructors.sort_by_key(|c| c.fields.len());
    for ctor in constructors {
        let mut args = Vec::new();
        let mut failed = false;
        for field in &ctor.fields {
            if let Some(default) = &field.default {
                args.push(format!("({default})"));
            } else if let Ok(value) = sample(&field.ty, ordinal, &mut visiting.clone()) {
                args.push(format!("({value})"));
            } else {
                failed = true;
                break;
            }
        }
        if !failed {
            visiting.remove(&ty);
            let base = ty.split_whitespace().next().unwrap_or(&ty);
            return Ok(format!("Semantics.{base}.{} {}", ctor.name, args.join(" ")));
        }
    }
    Err(LowerError::new(format!("no finite sample for {ty}")))
}
