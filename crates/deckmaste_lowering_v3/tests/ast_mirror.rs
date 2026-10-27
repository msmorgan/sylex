use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_semantics_v3::reader::Token;
use deckmaste_semantics_v3::reader::tokenize;

fn id(token: &Token) -> Option<&str> {
    match token {
        Token::Ident(name) => Some(name.as_str()),
        _ => None,
    }
}
fn variant(name: &str) -> String {
    if name == "self" {
        return "Self_".into();
    }
    name.trim_end_matches('_')
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().to_string() + chars.as_str()
            })
        })
        .collect()
}
fn rust_declarations(source: &str) -> BTreeMap<String, BTreeMap<String, usize>> {
    syn::parse_file(source)
        .unwrap()
        .items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Enum(item) => Some((
                item.ident.to_string(),
                item.variants
                    .iter()
                    .map(|v| (v.ident.to_string(), v.fields.len()))
                    .collect(),
            )),
            syn::Item::Struct(item) => Some((
                item.ident.to_string(),
                BTreeMap::from([("fields".into(), item.fields.len())]),
            )),
            _ => None,
        })
        .collect()
}
fn lean_declarations(source: &str) -> BTreeMap<String, BTreeMap<String, usize>> {
    let tokens = tokenize(source).unwrap();
    let mut declarations = BTreeMap::new();
    let mut index = 0;
    while index + 2 < tokens.len() {
        let kind = id(&tokens[index]);
        if !matches!(kind, Some("inductive" | "structure")) {
            index += 1;
            continue;
        }
        let name = id(&tokens[index + 1]).unwrap().to_owned();
        index += 2;
        while id(&tokens[index]) != Some("where") {
            index += 1;
        }
        index += 1;
        let mut members = BTreeMap::new();
        if kind == Some("inductive") {
            while matches!(tokens.get(index),Some(Token::Atom(atom)) if atom == "|") {
                let name = variant(id(&tokens[index + 1]).unwrap());
                index += 2;
                let mut fields = 0;
                while let Some(Token::Group('(', binder)) = tokens.get(index) {
                    fields += binder
                        .iter()
                        .take_while(|t| !matches!(t,Token::Atom(s) if s == ":"))
                        .count();
                    index += 1;
                }
                assert!(members.insert(name, fields).is_none());
            }
        } else {
            let mut count = 0;
            while index + 1 < tokens.len()
                && id(&tokens[index]).is_some()
                && matches!(&tokens[index+1],Token::Atom(s) if s == ":")
            {
                count += 1;
                index += 2;
                while index + 1 < tokens.len() {
                    if id(&tokens[index]).is_some()
                        && matches!(&tokens[index+1],Token::Atom(s) if s == ":")
                    {
                        break;
                    }
                    if matches!(
                        id(&tokens[index]),
                        Some(
                            "deriving"
                                | "inductive"
                                | "structure"
                                | "end"
                                | "def"
                                | "semantic_definition"
                        )
                    ) {
                        break;
                    }
                    index += 1;
                }
            }
            members.insert("fields".into(), count);
        }
        declarations.insert(name, members);
    }
    declarations
}
#[test]
fn discourse_state_and_refusals_keep_the_lean_data_shapes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lean/Semantics/Check");
    let mut rust = rust_declarations(include_str!("../src/reads.rs"));
    rust.remove("StepPhase");
    assert_eq!(
        rust,
        lean_declarations(&std::fs::read_to_string(root.join("Words.lean")).unwrap())
    );
    assert_eq!(
        rust_declarations(include_str!("../src/refusal.rs")),
        lean_declarations(&std::fs::read_to_string(root.join("Refusal.lean")).unwrap())
    );
}
