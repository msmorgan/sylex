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
        let samples = self.sample_invocations(key)?;
        Ok((samples.arguments[0].clone(), samples.result_type))
    }

    /// Sample explicit parameters, including defaults, across finite constructors.
    /// Constructor fields use deterministic base values. Default omission is also
    /// sampled. Recursive constructors and capped combinations are reported in
    /// `limitations`; these snapshots are a bounded comparison, not a proof.
    ///
    /// # Errors
    /// Reports unsupported implicit parameters and types without finite samples.
    pub fn sample_invocations(&self, key: &BundleKey) -> Result<InvocationSamples> {
        let decl = &self
            .bundle
            .items
            .get(key)
            .ok_or_else(|| LowerError::new("unknown declaration key"))?
            .declaration;
        let mut types = Vec::new();
        let mut result_type = None;
        let mut prefix = Vec::new();
        for p in decl.parameters.iter().filter(|p| p.implicit) {
            if reader::render(&p.ty) != "Type" {
                return Err(LowerError::new("unsupported implicit sample parameter"));
            }
            types.push(p.name);
            prefix.push(format!("({} := Nat)", p.name));
            if p.name == decl.return_type {
                result_type = Some(Ident::new("Nat"));
            }
        }
        let mut limitations = BTreeSet::new();
        let mut parameters = Vec::new();
        for (index, p) in decl.parameters.iter().filter(|p| !p.implicit).enumerate() {
            let opt_param =
                matches!(p.ty.first(), Some(reader::Token::Ident(n)) if n.as_str() == "optParam");
            let mut ty = reader::render(if opt_param { &p.ty[1..2] } else { &p.ty });
            for name in &types {
                ty = ty.replace(name.as_str(), "Nat");
            }
            let values = variants(&ty, index, &mut limitations)?;
            let mut args = Vec::new();
            if p.default.is_some() || opt_param {
                args.push(String::new());
            }
            args.extend(
                values
                    .into_iter()
                    .map(|value| format!("({} := {value})", p.name)),
            );
            parameters.push(args);
        }
        let total = parameters
            .iter()
            .fold(1usize, |n, p| n.saturating_mul(p.len()));
        let mut combinations = BTreeSet::new();
        let base = vec![0; parameters.len()];
        combinations.insert(base.clone());
        // Preserve every individual parameter variant before filling the product.
        for (i, values) in parameters.iter().enumerate() {
            for variant in 1..values.len() {
                let mut indices = base.clone();
                indices[i] = variant;
                combinations.insert(indices);
            }
        }
        if combinations.len() > SAMPLE_LIMIT {
            return Err(LowerError::new(format!(
                "individual parameter variants exceed sample cap {SAMPLE_LIMIT}"
            )));
        }
        let mut indices = base;
        loop {
            if combinations.len() >= SAMPLE_LIMIT {
                break;
            }
            combinations.insert(indices.clone());
            let mut next = false;
            for i in (0..parameters.len()).rev() {
                indices[i] += 1;
                if indices[i] < parameters[i].len() {
                    next = true;
                    break;
                }
                indices[i] = 0;
            }
            if !next {
                break;
            }
        }
        if total > combinations.len() {
            limitations.insert(format!("sample cap {SAMPLE_LIMIT}: {} of {total} combinations omitted; individual variants retained, remaining combinations in lexicographic parameter order omitted", total - combinations.len()));
        }
        let arguments = combinations
            .into_iter()
            .map(|indices| {
                prefix
                    .iter()
                    .cloned()
                    .chain(
                        indices
                            .into_iter()
                            .enumerate()
                            .map(|(i, j)| parameters[i][j].clone()),
                    )
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        Ok(InvocationSamples {
            arguments,
            result_type,
            limitations: limitations.into_iter().collect(),
        })
    }
}

const SAMPLE_LIMIT: usize = 256;

/// Invocations and the boundaries of their deterministic constructor basis.
pub struct InvocationSamples {
    pub arguments: Vec<String>,
    pub result_type: Option<Ident>,
    pub limitations: Vec<String>,
}

fn variants(ty: &str, ordinal: usize, limitations: &mut BTreeSet<String>) -> Result<Vec<String>> {
    let ty = normal(ty);
    if ty == "Bool" {
        return Ok(vec!["false".into(), "true".into()]);
    }
    if let Some(value) = product_sample(&ty, ordinal)? {
        return Ok(vec![value]);
    }
    if let Some(inner) = ty.strip_prefix("Option ") {
        let finite = inner == "Bool"
            || shape(inner).is_some_and(|ctors| ctors.iter().all(|ctor| ctor.fields.is_empty()));
        let present = if finite {
            variants(inner, ordinal, limitations)?
        } else {
            vec![sample(inner, ordinal, &mut BTreeSet::new())?]
        };
        let mut values = vec!["none".into()];
        values.extend(present.into_iter().map(|value| format!("some ({value})")));
        return Ok(values);
    }
    if let Some(inner) = ty.strip_prefix("List ") {
        let elements = variants(inner, ordinal, limitations)?;
        let mut values = vec!["[]".into()];
        values.extend(elements.iter().map(|value| format!("[{value}]")));
        if elements.len() > 1 {
            values.push(format!("[{}, {}]", elements[0], elements[1]));
        }
        return Ok(values);
    }
    if let Some((input, output)) = ty.split_once('→').or_else(|| ty.split_once("->")) {
        let outputs = variants(output, ordinal, limitations)?;
        let mut values = outputs
            .iter()
            .map(|value| format!("fun (sample : {input}) => {value}"))
            .collect::<Vec<_>>();
        if let Some(constructors) = shape(input)
            && let Some(first) = constructors.first()
            && constructors.len() > 1
            && outputs.len() > 1
        {
            let input = normal(input);
            let pattern = format!(
                "Semantics.{input}.{} {}",
                first.name,
                vec!["_"; first.fields.len()].join(" ")
            );
            for i in 0..outputs.len() {
                values.push(format!(
                    "fun (sample : {input}) => match sample with | {pattern} => {} | x => {}",
                    outputs[i],
                    outputs[(i + 1) % outputs.len()]
                ));
            }
        }
        Ok(values)
    } else {
        constructor_variants(&ty, ordinal, limitations)
    }
}

fn constructor_variants(
    ty: &str,
    ordinal: usize,
    limitations: &mut BTreeSet<String>,
) -> Result<Vec<String>> {
    let ty = normal(ty);
    if matches!(ty.as_str(), "Nat" | "Int" | "String") {
        return Ok(vec![sample(&ty, ordinal, &mut BTreeSet::new())?]);
    }
    let Some(mut constructors) = shape(&ty) else {
        return Ok(vec![sample(&ty, ordinal, &mut BTreeSet::new())?]);
    };
    constructors.sort_by_key(|c| c.fields.len());
    let mut values = vec![sample(&ty, ordinal, &mut BTreeSet::new())?];
    for ctor in constructors {
        match constructor_sample(&ty, &ctor, ordinal, &BTreeSet::from([ty.clone()])) {
            Ok(value) => {
                if !values.contains(&value) {
                    values.push(value);
                }
                if ctor.rust_name.as_str().is_empty() {
                    // Structure defaults alone never exercise populated records.
                    for field in &ctor.fields {
                        for value in variants(&field.ty, ordinal, limitations)? {
                            let mut changed = ctor.clone();
                            let target = changed
                                .fields
                                .iter_mut()
                                .find(|f| f.name == field.name)
                                .unwrap();
                            target.default = Some(value);
                            let value = constructor_sample(
                                &ty,
                                &changed,
                                ordinal,
                                &BTreeSet::from([ty.clone()]),
                            )?;
                            if !values.contains(&value) {
                                values.push(value);
                            }
                        }
                    }
                }
            }
            Err(error) => {
                limitations.insert(format!("{ty}.{} omitted: {error}", ctor.name));
            }
        }
    }
    if values.is_empty() {
        return Err(LowerError::new(format!("no finite sample for {ty}")));
    }
    Ok(values)
}

fn constructor_sample(
    ty: &str,
    ctor: &super::Constructor,
    ordinal: usize,
    visiting: &BTreeSet<String>,
) -> Result<String> {
    let args = ctor
        .fields
        .iter()
        .map(|field| {
            let value = if let Some(default) = &field.default {
                default.clone()
            } else {
                sample(&field.ty, ordinal, &mut visiting.clone())?
            };
            Ok(format!("({value})"))
        })
        .collect::<Result<Vec<_>>>()?;
    let base = ty.split_whitespace().next().unwrap_or(ty);
    Ok(format!(
        "Semantics.{base}.{}{}",
        ctor.name,
        if args.is_empty() { String::new() } else { format!(" {}", args.join(" ")) }
    ))
}

fn sample(ty: &str, ordinal: usize, visiting: &mut BTreeSet<String>) -> Result<String> {
    let ty = normal(ty);
    if visiting.contains(&ty) {
        return Err(LowerError::new(format!("recursive sample type {ty}")));
    }
    match ty.as_str() {
        "Nat" | "Int" | "Amount" => return Ok((ordinal + 3).to_string()),
        "String" => return Ok(format!("\"Sample {ordinal}\"")),
        "Bool" => return Ok("false".into()),
        "NounPhrase" => return Ok("you".into()),
        "Cost" => return Ok("[2]".into()),
        _ => {}
    }
    if let Some(value) = product_sample(&ty, ordinal)? {
        return Ok(value);
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
        if let Ok(value) = constructor_sample(&ty, &ctor, ordinal, visiting) {
            visiting.remove(&ty);
            return Ok(value);
        }
    }
    Err(LowerError::new(format!("no finite sample for {ty}")))
}

fn product_sample(ty: &str, ordinal: usize) -> Result<Option<String>> {
    let mut depth = 0;
    for (i, c) in ty.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '×' if depth == 0 => {
                return Ok(Some(format!(
                    "({}, {})",
                    sample(&ty[..i], ordinal, &mut BTreeSet::new())?,
                    sample(&ty[i + c.len_utf8()..], ordinal, &mut BTreeSet::new())?
                )));
            }
            _ => {}
        }
    }
    Ok(None)
}
