//! Pure data computation used by authored helpers.
use super::Bound;
use super::BundleKey;
use super::Elaborator;
use super::LowerError;
use super::Result;
use super::Scope;
use super::Token;
use super::Value;
use super::bare;
use super::construct;
use super::is_atom;
use super::normal;
use super::reader;
use super::shape;
use super::split;

fn word(token: &Token, value: &str) -> bool {
    matches!(token, Token::Ident(n) if n.as_str() == value)
}
fn product(ty: &str) -> Option<(&str, &str)> {
    let mut depth = 0;
    for (i, c) in ty.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '×' if depth == 0 => return Some((&ty[..i], &ty[i + c.len_utf8()..])),
            _ => {}
        }
    }
    None
}
impl Elaborator<'_> {
    fn type_of(&self, tokens: &[Token], scope: &Scope) -> Result<String> {
        if let [Token::Group('(', inner)] = tokens {
            return self.type_of(inner, scope);
        }
        if let [Token::String(_)] = tokens {
            return Ok("String".into());
        }
        if let [Token::Atom(n)] = tokens
            && n.parse::<u64>().is_ok()
        {
            return Ok("Nat".into());
        }
        if let [Token::Group('[', items)] = tokens
            && let Some(first) = split(items, ",").first()
        {
            return Ok(format!("List ({})", self.type_of(first, scope)?));
        }
        if tokens.first().is_some_and(|t| word(t, "if")) {
            let then = tokens
                .iter()
                .position(|t| word(t, "then"))
                .ok_or_else(|| LowerError::new("if requires then"))?;
            let otherwise = tokens
                .iter()
                .position(|t| word(t, "else"))
                .ok_or_else(|| LowerError::new("if requires else"))?;
            return self
                .type_of(&tokens[then + 1..otherwise], scope)
                .or_else(|_| self.type_of(&tokens[otherwise + 1..], scope));
        }
        if tokens.first().is_some_and(|t| word(t, "match")) {
            let with = tokens
                .iter()
                .position(|t| word(t, "with"))
                .ok_or_else(|| LowerError::new("match requires with"))?;
            let subject = self.type_of(&tokens[1..with], scope)?;
            for branch in split(&tokens[with + 1..], "|") {
                let arrow = branch
                    .iter()
                    .position(|t| is_atom(t, "=>"))
                    .ok_or_else(|| LowerError::new("branch requires =>"))?;
                let mut local = scope.clone();
                let name = branch
                    .iter()
                    .find_map(|t| if let Token::Ident(n) = t { Some(n.as_str()) } else { None });
                if let Some(ctor) = shape(&subject).and_then(|cs| {
                    cs.into_iter()
                        .find(|c| Some(c.name.as_str()) == name.map(bare))
                }) {
                    let symbolic = ctor.value(vec![Value::Null; ctor.fields.len()]);
                    bind_pattern(&branch[..arrow], &subject, &symbolic, &mut local)?;
                    if let Ok(ty) = self.type_of(&branch[arrow + 1..], &local) {
                        return Ok(ty);
                    }
                }
            }
        }
        let Some(Token::Ident(name)) = tokens.first() else {
            return Err(LowerError::new("expression requires an expected type"));
        };
        if let Some(bound) = scope.bindings.get(name) {
            return Ok(bound.ty.clone());
        }
        if let Some(function) = scope.functions.get(name) {
            return Ok(function.output.clone());
        }
        match name.as_str() {
            "Semantics.Characteristics.valueEdits" => return Ok("List CharacteristicEdit".into()),
            "Semantics.Characteristics.typeChanges" => return Ok("TypeLineChanges".into()),
            "Option.some" | "some" => {
                return Ok(format!("Option ({})", self.type_of(&tokens[1..], scope)?));
            }
            "List.length" => return Ok("Nat".into()),
            "List.isEmpty" => return Ok("Bool".into()),
            "List.map" | "List.filterMap" if tokens.len() == 3 => {
                let result = self.function_result(&tokens[1..2], scope)?;
                let result = if name.as_str() == "List.filterMap" {
                    normal(&result)
                        .strip_prefix("Option ")
                        .map(str::to_owned)
                        .ok_or_else(|| {
                            LowerError::new("filterMap function must return an option")
                        })?
                } else {
                    result
                };
                return Ok(format!("List ({result})"));
            }
            _ => {}
        }
        if let Some(field_name) = name.as_str().strip_prefix("Semantics.Characteristics.")
            && let Some(field) = shape("Characteristics")
                .and_then(|cs| cs.into_iter().next())
                .and_then(|c| c.fields.into_iter().find(|f| f.name.as_str() == field_name))
        {
            return Ok(field.ty);
        }
        let candidates = self
            .bundle
            .items
            .values()
            .filter(|i| i.declaration.name.as_str() == bare(name.as_str()))
            .map(|i| normal(i.declaration.return_type.as_str()))
            .collect::<std::collections::BTreeSet<_>>();
        if candidates.len() == 1 {
            return Ok(candidates.into_iter().next().unwrap());
        }
        if let Some((prefix, _)) = name.as_str().rsplit_once('.') {
            let ty = bare(prefix);
            if shape(ty).is_some() {
                return Ok(ty.into());
            }
        }
        Err(LowerError::new("expression requires an expected type"))
    }
    fn function_result(&self, tokens: &[Token], scope: &Scope) -> Result<String> {
        if let [Token::Group('(', inner)] = tokens {
            return self.function_result(inner, scope);
        }
        if tokens.first().is_some_and(|t| word(t, "fun")) {
            let arrow = tokens
                .iter()
                .position(|t| is_atom(t, "=>"))
                .ok_or_else(|| LowerError::new("lambda requires =>"))?;
            let Some(Token::Group('(', binder)) = tokens.get(1) else {
                return Err(LowerError::new(
                    "type inference needs a typed lambda binder",
                ));
            };
            let [Token::Ident(name), Token::Atom(colon), ty @ ..] = binder.as_slice() else {
                return Err(LowerError::new("lambda binder requires a type"));
            };
            if colon != ":" {
                return Err(LowerError::new("lambda binder requires a type"));
            }
            let mut local = scope.clone();
            local.bindings.insert(
                *name,
                Bound {
                    ty: normal(&reader::render(ty)),
                    value: Value::Null,
                    identity: None,
                    caller: false,
                },
            );
            return self.type_of(&tokens[arrow + 1..], &local);
        }
        self.type_of(tokens, scope)
    }
    pub(super) fn infer(
        &self,
        tokens: &[Token],
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<(String, Value)> {
        if let [Token::Group('(', inner)] = tokens {
            return self.infer(inner, scope, stack);
        }
        if let [Token::Ident(n)] = tokens
            && let Some(bound) = scope.bindings.get(n)
        {
            return Ok((bound.ty.clone(), bound.value.clone()));
        }
        if let [Token::Group('[', items)] = tokens {
            let parts = split(items, ",");
            let Some(first) = parts.first() else {
                return Err(LowerError::new("empty list requires an expected type"));
            };
            let (element, _) = self.infer(first, scope, stack)?;
            let ty = format!("List ({element})");
            let value = self.eval(tokens, &ty, scope, stack)?;
            return Ok((ty, value));
        }
        if let Ok(ty) = self.type_of(tokens, scope) {
            let value = self.eval(tokens, &ty, scope, stack)?;
            return Ok((ty, value));
        }
        let Some(Token::Ident(name)) = tokens.first() else {
            return Err(LowerError::new("cannot infer type of this data expression"));
        };
        let candidates = self
            .bundle
            .items
            .values()
            .filter(|i| i.declaration.name.as_str() == bare(name.as_str()))
            .map(|i| normal(i.declaration.return_type.as_str()))
            .collect::<std::collections::BTreeSet<_>>();
        let mut values = Vec::new();
        for ty in candidates {
            let mut trial = scope.clone();
            if let Ok(value) = self.eval(tokens, &ty, &mut trial, stack) {
                values.push((ty, value, trial));
            }
        }
        if values.len() == 1 {
            let (ty, value, trial) = values.remove(0);
            *scope = trial;
            return Ok((ty, value));
        }
        Err(LowerError::new(format!(
            "cannot infer an unambiguous type for {}",
            reader::render(tokens)
        )))
    }
    pub(super) fn compute(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Option<Value>> {
        if let [base @ .., Token::Atom(dot), Token::Ident(field)] = tokens
            && dot == "."
            && field.as_str() == "plur"
            && base.len() == 1
        {
            let value = self.eval(base, "NounPhrase", scope, stack)?;
            let np: crate::ast::NounPhrase =
                serde_json::from_value(value).map_err(|e| LowerError::new(e.to_string()))?;
            return serde_json::to_value(np.plurality())
                .map(Some)
                .map_err(|e| LowerError::new(e.to_string()));
        }
        if let Some((left, right)) = product(ty)
            && let Some(i) = tokens.iter().position(|t| is_atom(t, ","))
        {
            return Ok(Some(Value::Array(vec![
                self.eval(&tokens[..i], left, scope, stack)?,
                self.eval(&tokens[i + 1..], right, scope, stack)?,
            ])));
        }
        if tokens.first().is_some_and(|t| is_atom(t, "⟨"))
            && tokens.last().is_some_and(|t| is_atom(t, "⟩"))
        {
            let ctor = shape(ty)
                .and_then(|cs| cs.into_iter().find(|c| c.name.as_str() == "mk"))
                .ok_or_else(|| LowerError::new(format!("{ty} has no anonymous constructor")))?;
            let args = split(&tokens[1..tokens.len() - 1], ",");
            if args.len() != ctor.fields.len() {
                return Err(LowerError::new("anonymous constructor arity mismatch"));
            }
            let values = args
                .iter()
                .zip(&ctor.fields)
                .map(|(t, f)| self.eval(t, &f.ty, scope, stack))
                .collect::<Result<_>>()?;
            return Ok(Some(ctor.value(values)));
        }
        if tokens
            .first()
            .is_some_and(|t| word(t, "have") || word(t, "let"))
        {
            let Some(assign) = tokens.iter().position(|t| is_atom(t, ":=")) else {
                return Err(LowerError::new("let requires :="));
            };
            let Some(end) = tokens.iter().position(|t| is_atom(t, ";")) else {
                return Err(LowerError::new("let requires ;"));
            };
            let Some(Token::Ident(name)) = tokens.get(1) else {
                return Err(LowerError::new("let requires a name"));
            };
            let (bound_type, value) = if tokens.get(2).is_some_and(|t| is_atom(t, ":")) {
                let bound_type = normal(&reader::render(&tokens[3..assign]));
                let value = self.eval(&tokens[assign + 1..end], &bound_type, scope, stack)?;
                (bound_type, value)
            } else {
                self.infer(&tokens[assign + 1..end], scope, stack)?
            };
            let mut local = scope.clone();
            local.bindings.insert(
                *name,
                Bound {
                    ty: bound_type,
                    value,
                    identity: None,
                    caller: false,
                },
            );
            let value = self.eval(&tokens[end + 1..], ty, &mut local, stack)?;
            scope.seen = local.seen;
            return Ok(Some(value));
        }
        if tokens.first().is_some_and(|t| word(t, "match")) {
            return self.match_data(tokens, ty, scope, stack).map(Some);
        }
        if tokens.first().is_some_and(|t| word(t, "if")) {
            let then = tokens
                .iter()
                .position(|t| word(t, "then"))
                .ok_or_else(|| LowerError::new("if requires then"))?;
            let otherwise = tokens
                .iter()
                .position(|t| word(t, "else"))
                .ok_or_else(|| LowerError::new("if requires else"))?;
            let condition = self.eval(&tokens[1..then], "Bool", scope, stack)?;
            let branch = if condition == Value::Bool(true) {
                &tokens[then + 1..otherwise]
            } else {
                &tokens[otherwise + 1..]
            };
            return self.eval(branch, ty, scope, stack).map(Some);
        }
        if ty == "Bool"
            && let Some(i) = tokens.iter().position(|t| is_atom(t, "=="))
        {
            let (operand_type, left) = self.infer(&tokens[..i], scope, stack)?;
            let right = self.eval(&tokens[i + 1..], &operand_type, scope, stack)?;
            return Ok(Some(Value::Bool(left == right)));
        }
        if ty == "Bool" {
            if let Some(i) = tokens.iter().position(|t| is_atom(t, "=")) {
                let left = self.eval(&tokens[..i], "Bool", scope, stack)?;
                let right = self.eval(&tokens[i + 1..], "Bool", scope, stack)?;
                return Ok(Some(Value::Bool(left == right)));
            }
            for op in ["&&", "||"] {
                if let Some(i) = tokens.iter().position(|t| is_atom(t, op)) {
                    let left = self.eval(&tokens[..i], "Bool", scope, stack)? == Value::Bool(true);
                    let right =
                        self.eval(&tokens[i + 1..], "Bool", scope, stack)? == Value::Bool(true);
                    return Ok(Some(Value::Bool(if op == "&&" {
                        left && right
                    } else {
                        left || right
                    })));
                }
            }
        }
        if let Some(inner) = ty.strip_prefix("List ")
            && let Some(i) = tokens.iter().position(|t| is_atom(t, "::"))
        {
            let head = self.eval(&tokens[..i], inner, scope, stack)?;
            let Value::Array(mut tail) = self.eval(&tokens[i + 1..], ty, scope, stack)? else {
                return Err(LowerError::new("cons requires list tail"));
            };
            tail.insert(0, head);
            return Ok(Some(Value::Array(tail)));
        }
        self.helper(tokens, ty, scope, stack)
    }
    pub(super) fn apply(
        &self,
        function: &[Token],
        argument: Bound,
        output: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        if let [Token::Group('(', inner)] = function {
            return self.apply(inner, argument, output, scope, stack);
        }
        let mut local = scope.clone();
        if function.first().is_some_and(|t| word(t, "fun")) {
            let arrow = function
                .iter()
                .position(|t| is_atom(t, "=>"))
                .ok_or_else(|| LowerError::new("lambda requires =>"))?;
            let binder = match &function[1..arrow] {
                [Token::Group('(', inner)] => inner.as_slice(),
                other => other,
            };
            let Some(Token::Ident(name)) = binder.first() else {
                return Err(LowerError::new("lambda requires a binder"));
            };
            local.bindings.insert(*name, argument);
            return self.eval(&function[arrow + 1..], output, &mut local, stack);
        }
        let mut index = 0;
        let name = loop {
            let name = dpsi::Ident::new(&format!("__argument_{index}"));
            if !local.bindings.contains_key(&name) {
                break name;
            }
            index += 1;
        };
        local.bindings.insert(name, argument);
        let mut call = function.to_vec();
        call.push(Token::Ident(name));
        self.eval(&call, output, &mut local, stack)
    }

    fn match_data(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Value> {
        let with = tokens
            .iter()
            .position(|t| word(t, "with"))
            .ok_or_else(|| LowerError::new("match requires with"))?;
        let (subject_type, value) = self.infer(&tokens[1..with], scope, stack)?;
        for branch in split(&tokens[with + 1..], "|") {
            let arrow = branch
                .iter()
                .position(|t| is_atom(t, "=>"))
                .ok_or_else(|| LowerError::new("match branch requires =>"))?;
            let pattern = &branch[..arrow];
            let mut local = scope.clone();
            if bind_pattern(pattern, &subject_type, &value, &mut local)? {
                let value = self.eval(&branch[arrow + 1..], ty, &mut local, stack)?;
                scope.seen = local.seen;
                return Ok(value);
            }
        }
        Err(LowerError::new("nonexhaustive data match"))
    }
    fn helper(
        &self,
        tokens: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Option<Value>> {
        let Some(Token::Ident(name)) = tokens.first() else {
            return Ok(None);
        };
        let args = &tokens[1..];
        if let Some(field_name) = name.as_str().strip_prefix("Semantics.Characteristics.")
            && let Some(field) = shape("Characteristics")
                .and_then(|cs| cs.into_iter().next())
                .and_then(|c| c.fields.into_iter().find(|f| f.name.as_str() == field_name))
        {
            let value = self.eval(args, "Characteristics", scope, stack)?;
            return Self::coerce(value[field.rust_name.as_str()].clone(), &field.ty, ty).map(Some);
        }
        match name.as_str() {
            "keyword" | "Semantics.Templates.keyword" if ty == "Ability" && !args.is_empty() => {
                let label = self.eval(args, "KeywordLabel", scope, stack)?;
                construct(
                    "Ability",
                    "keyword",
                    vec![label, Value::Array(vec![]), Value::Array(vec![])],
                )
                .map(Some)
            }
            "List.length" | "List.isEmpty" => {
                let (_, value) = self.infer(args, scope, stack)?;
                let values = value
                    .as_array()
                    .ok_or_else(|| LowerError::new("list helper requires a list"))?;
                Ok(Some(if name.as_str() == "List.length" {
                    Value::from(values.len())
                } else {
                    Value::Bool(values.is_empty())
                }))
            }
            "Semantics.Characteristics.typeChanges" => {
                let value = self.eval(args, "Characteristics", scope, stack)?;
                let c =
                    serde_json::from_value(value).map_err(|e| LowerError::new(e.to_string()))?;
                serde_json::to_value(super::characteristics::type_changes(&c))
                    .map(Some)
                    .map_err(|e| LowerError::new(e.to_string()))
            }
            "Semantics.Characteristics.valueEdits" if args.len() == 2 => {
                let c = serde_json::from_value(self.eval(
                    &args[..1],
                    "Characteristics",
                    scope,
                    stack,
                )?)
                .map_err(|e| LowerError::new(e.to_string()))?;
                let op =
                    serde_json::from_value(self.eval(&args[1..], "QualityOp", scope, stack)?)
                        .map_err(|e| LowerError::new(e.to_string()))?;
                serde_json::to_value(super::characteristics::value_edits(&c, &op))
                    .map(Some)
                    .map_err(|e| LowerError::new(e.to_string()))
            }
            "Semantics.Location.observedOnDeparture" => {
                let value = self.eval(args, "Location", scope, stack)?;
                let location =
                    serde_json::from_value(value).map_err(|e| LowerError::new(e.to_string()))?;
                Ok(Some(Value::Bool(observed_departure(&location))))
            }
            "Semantics.Cost.asPaid" => {
                let value = self.eval(args, "Cost", scope, stack)?;
                let cost =
                    serde_json::from_value(value).map_err(|e| LowerError::new(e.to_string()))?;
                Ok(Some(
                    serde_json::to_value(as_paid(cost))
                        .map_err(|e| LowerError::new(e.to_string()))?,
                ))
            }
            "Semantics.QualityPayload.edits" if args.len() == 2 => {
                let op =
                    serde_json::from_value(self.eval(&args[..1], "QualityOp", scope, stack)?)
                        .map_err(|e| LowerError::new(e.to_string()))?;
                let payload: super::characteristics::QualityPayload = serde_json::from_value(
                    self.eval(&args[1..], "QualityPayload", scope, stack)?,
                )
                .map_err(|e| LowerError::new(e.to_string()))?;
                serde_json::to_value(payload.edits(op))
                    .map(Some)
                    .map_err(|e| LowerError::new(e.to_string()))
            }
            _ => self.collection_helper(name.as_str(), args, ty, scope, stack),
        }
    }
    fn collection_helper(
        &self,
        name: &str,
        args: &[Token],
        ty: &str,
        scope: &mut Scope,
        stack: &mut Vec<BundleKey>,
    ) -> Result<Option<Value>> {
        match name {
            "Option.map" if args.len() == 2 => {
                let (source_type, source) = self.infer(&args[1..], scope, stack)?;
                let input = normal(&source_type)
                    .strip_prefix("Option ")
                    .map(str::to_owned)
                    .ok_or_else(|| LowerError::new("Option.map requires an option"))?;
                let output = ty
                    .strip_prefix("Option ")
                    .ok_or_else(|| LowerError::new("Option.map returns an option"))?;
                if source.is_null() {
                    return Ok(Some(Value::Null));
                }
                self.apply(
                    &args[..1],
                    Bound {
                        ty: input,
                        value: source,
                        identity: None,
                        caller: false,
                    },
                    output,
                    scope,
                    stack,
                )
                .map(Some)
            }
            "List.map" | "List.filterMap" if args.len() == 2 => {
                let (source_type, source) = self.infer(&args[1..], scope, stack)?;
                let input = normal(&source_type)
                    .strip_prefix("List ")
                    .map(str::to_owned)
                    .ok_or_else(|| LowerError::new("List.map requires a list"))?;
                let output = ty
                    .strip_prefix("List ")
                    .ok_or_else(|| LowerError::new("List.map returns a list"))?;
                let output = if name == "List.filterMap" {
                    format!("Option ({output})")
                } else {
                    output.into()
                };
                let Value::Array(items) = source else {
                    return Err(LowerError::new("invalid list value"));
                };
                let values = items
                    .into_iter()
                    .map(|value| {
                        self.apply(
                            &args[..1],
                            Bound {
                                ty: input.clone(),
                                value,
                                identity: None,
                                caller: false,
                            },
                            &output,
                            scope,
                            stack,
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;
                let values = if name == "List.filterMap" {
                    values.into_iter().filter(|v| !v.is_null()).collect()
                } else {
                    values
                };
                Ok(Some(Value::Array(values)))
            }
            "Option.getD" if args.len() == 2 => {
                let option = self.eval(&args[..1], &format!("Option ({ty})"), scope, stack)?;
                Ok(Some(if option.is_null() {
                    self.eval(&args[1..], ty, scope, stack)?
                } else {
                    option
                }))
            }
            _ => Ok(None),
        }
    }
}
fn bind_pattern(pattern: &[Token], ty: &str, value: &Value, scope: &mut Scope) -> Result<bool> {
    let normalized = normal(ty);
    let ty = normalized.as_str();
    if let [Token::Group('(', inner)] = pattern {
        return bind_pattern(inner, ty, value, scope);
    }
    if let Some(inner) = ty.strip_prefix("List ")
        && let Some(i) = pattern.iter().position(|t| is_atom(t, "::"))
    {
        let Some(values) = value.as_array().filter(|v| !v.is_empty()) else {
            return Ok(false);
        };
        return Ok(bind_pattern(&pattern[..i], inner, &values[0], scope)?
            && bind_pattern(
                &pattern[i + 1..],
                ty,
                &Value::Array(values[1..].to_vec()),
                scope,
            )?);
    }
    if let Some((left, right)) = product(ty)
        && let Some(i) = pattern.iter().position(|t| is_atom(t, ","))
    {
        return Ok(bind_pattern(&pattern[..i], left, &value[0], scope)?
            && bind_pattern(&pattern[i + 1..], right, &value[1], scope)?);
    }
    let (name, args) = match pattern {
        [Token::Ident(n), rest @ ..] => (n, rest),
        [Token::Atom(dot), Token::Ident(n), rest @ ..] if dot == "." => (n, rest),
        _ => return Err(LowerError::new("invalid data pattern")),
    };
    if let Some(ctor) = shape(ty).and_then(|cs| {
        cs.into_iter()
            .find(|c| c.name.as_str() == bare(name.as_str()))
    }) {
        if args.len() != ctor.fields.len() {
            return Err(LowerError::new("constructor pattern arity mismatch"));
        }
        if ctor.fields.is_empty() {
            return Ok(value == ctor.rust_name.as_str());
        }
        let Some(fields) = value.get(ctor.rust_name.as_str()) else {
            return Ok(false);
        };
        for (arg, field) in args.iter().zip(ctor.fields) {
            if !bind_pattern(
                std::slice::from_ref(arg),
                &field.ty,
                &fields[field.rust_name.as_str()],
                scope,
            )? {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    if !args.is_empty() {
        return Err(LowerError::new("unknown constructor in pattern"));
    }
    if name.as_str() != "_" {
        scope.bindings.insert(
            *name,
            Bound {
                ty: normal(ty),
                value: value.clone(),
                identity: None,
                caller: false,
            },
        );
    }
    Ok(true)
}
fn observed_departure(location: &crate::ast::Location) -> bool {
    use crate::ast::Location;
    use crate::ast::Zone;
    match location {
        Location::InCaller { body } | Location::FromEnd { loc: body, .. } => {
            observed_departure(body)
        }
        Location::Zone { zone, .. } => *zone == Zone::Battlefield,
        Location::Coord { locs, .. } => locs.iter().any(observed_departure),
        _ => false,
    }
}
fn as_paid(cost: crate::ast::Cost) -> crate::ast::Cost {
    use crate::ast::Action;
    use crate::ast::Cost;
    use crate::ast::PayTimes;
    use crate::ast::PaymentOutcome;
    match cost {
        Cost::Perform { action } => match *action {
            Action::Pay {
                cost,
                times: PayTimes::Once,
                outcome: PaymentOutcome::Paid,
                whose: None,
                keyword: None,
            } => *cost,
            action => Cost::Perform {
                action: Box::new(action),
            },
        },
        Cost::InCaller { body } => Cost::InCaller {
            body: Box::new(as_paid(*body)),
        },
        Cost::Scaled { cost, amount } => Cost::Scaled {
            cost: Box::new(as_paid(*cost)),
            amount,
        },
        Cost::Compound { costs } => Cost::Compound {
            costs: costs.into_iter().map(as_paid).collect(),
        },
        other => other,
    }
}
