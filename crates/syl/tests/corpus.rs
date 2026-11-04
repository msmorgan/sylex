//! Every `.syl` file under `plugins_v3/` parses.

use std::path::Path;
use std::path::PathBuf;

use sylex_syl::Entry;
use sylex_syl::File;
use sylex_syl::FileId;
use sylex_syl::Item;
use sylex_syl::Shape;
use sylex_syl::Value;
use sylex_syl::ValueKind;
use sylex_syl::lex::TokenKind;
use sylex_syl::lex::lex;
use sylex_syl::parse;

/// Counts the comment items in `items`, and in the nodes `nested` reaches.
fn count_items<T>(items: &[Item<T>], nested: impl Fn(&T) -> usize) -> usize {
    items
        .iter()
        .map(|item| match &item.entry {
            Entry::Comment(_) => 1,
            Entry::Node(node) => nested(node),
        })
        .sum()
}

fn value_comments(value: &Value) -> usize {
    match &value.kind {
        ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => 0,
        ValueKind::List(items) => count_items(items, value_comments),
        ValueKind::Record(record) => count_items(&record.fields, |f| value_comments(&f.value)),
        ValueKind::Call(call) => count_items(&call.args, |a| value_comments(&a.value)),
    }
}

fn file_comments(file: &File) -> usize {
    count_items(&file.items, |d| match &d.shape {
        Shape::Nameless(value) => value_comments(value),
        Shape::Named(named) => {
            count_items(&named.params, |p| {
                p.default.as_ref().map_or(0, value_comments)
            }) + named
                .body
                .as_ref()
                .map_or(0, |r| count_items(&r.fields, |f| value_comments(&f.value)))
        }
    })
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("readable directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "syl") {
            out.push(path);
        }
    }
}

#[test]
fn every_plugins_v3_file_parses() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3");
    let mut paths = Vec::new();
    collect(&root, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no .syl files under {}", root.display());
    let mut failures = Vec::new();
    for (index, path) in paths.iter().enumerate() {
        let source = std::fs::read_to_string(path).expect("readable .syl file");
        let id = FileId(u32::try_from(index).expect("fewer than 2^32 files"));
        match parse(id, &source) {
            Ok(file) => {
                assert!(
                    file.declarations().next().is_some(),
                    "{}: no declarations",
                    path.display()
                );
                // Every declaration in the corpus is a `card` exactly when it
                // is nameless; the shape is decided without the kind word.
                for declaration in file.declarations() {
                    assert_eq!(
                        declaration.kind.ident == "card",
                        matches!(declaration.shape, Shape::Nameless(_)),
                        "{}: wrong shape for `{}`",
                        path.display(),
                        declaration.kind.ident
                    );
                }
                let lexed = lex(id, &source).expect("parsed source lexes");
                let comments = lexed
                    .iter()
                    .filter(|t| matches!(t.kind, TokenKind::Comment(_)))
                    .count();
                assert_eq!(
                    file_comments(&file),
                    comments,
                    "{}: comments lost",
                    path.display()
                );
            }
            Err(error) => failures.push(error.render(&path.display().to_string(), &source)),
        }
    }
    assert!(
        failures.is_empty(),
        "{} files failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
