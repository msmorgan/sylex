//! `cargo xtask dsl`: `.syl` tooling. `fmt` lives in [`crate::dsl_fmt`].
//!
//! `cargo xtask dsl differential` checks that `sylex_syl` and `SylGen`'s
//! parser (`lean/SylGen/Parser.lean`) read every `plugins/**/*.syl` file
//! identically.
//!
//! The Lean executable `sylParseDump` writes `SylGen`'s `Declaration`s, before
//! resolution, as JSON to a gitignored path under `lean/.lake/build`. This
//! command builds and runs it, parses the same files with `sylex_syl`,
//! projects each Rust tree onto the dump's shape and compares file by file. The
//! projection rebuilds what `SylGen`'s parser derives rather than reads: the
//! sort it fills in from the kind (`kindSort`), a nameless `card`'s printed
//! name and card value, and the doc comment as `SylGen`'s `documentation`
//! string.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context as _;
use clap::Args;
use clap::Subcommand;
use sylex_syl::File;
use sylex_syl::FileId;
use sylex_syl::Shape;
use sylex_syl::Sort;
use sylex_syl::Value;
use sylex_syl::ValueKind;
use serde_json::Value as Json;
use serde_json::json;

#[derive(Debug, Args)]
pub struct DslArgs {
    #[command(subcommand)]
    command: DslCmd,
}

#[derive(Debug, Subcommand)]
enum DslCmd {
    /// Parse every `plugins` `.syl` file with both `sylex_syl` and
    /// `SylGen`'s parser, and fail on any difference between the trees.
    Differential,
    /// Format `.syl` files (default: every `plugins` file) in place, check
    /// them, or format standard input.
    Fmt(crate::dsl_fmt::FmtArgs),
}

/// Where `sylParseDump` writes, relative to `lean/`.
const DUMP: &str = ".lake/build/syl-parse-dump.json";

pub fn run(args: &DslArgs) -> anyhow::Result<()> {
    match &args.command {
        DslCmd::Differential => differential(),
        DslCmd::Fmt(args) => crate::dsl_fmt::run(args),
    }
}

fn differential() -> anyhow::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let workbench = root.join("lean");
    let status = Command::new("lake")
        .current_dir(&workbench)
        .args(["build", "-q", "sylParseDump"])
        .status()
        .context("starting the sylParseDump build")?;
    anyhow::ensure!(status.success(), "building sylParseDump failed: {status}");
    let status = Command::new("lake")
        .current_dir(&workbench)
        .args(["exe", "sylParseDump"])
        .arg(&root)
        .arg(DUMP)
        .status()
        .context("starting sylParseDump")?;
    anyhow::ensure!(status.success(), "sylParseDump failed: {status}");
    let dump_path = workbench.join(DUMP);
    let dump = std::fs::read_to_string(&dump_path)
        .with_context(|| format!("reading {}", dump_path.display()))?;
    let lean: BTreeMap<String, Json> =
        serde_json::from_str(&dump).with_context(|| format!("decoding {}", dump_path.display()))?;
    let rust = parse_corpus(&root)?;
    let differences = compare(&lean, &rust);
    if differences.is_empty() {
        println!("dsl differential: {} files, no differences", rust.len());
        Ok(())
    } else {
        for difference in &differences {
            eprintln!("{difference}");
        }
        anyhow::bail!(
            "dsl differential: {} of {} files differ",
            differences.len(),
            lean.keys()
                .chain(rust.keys())
                .collect::<BTreeSet<_>>()
                .len()
        )
    }
}

/// Parses every `plugins/**/*.syl` file under `root` with `sylex_syl`,
/// keyed by its `/`-separated path relative to `root`; each entry is the
/// projected declarations or the rendered parse error.
fn parse_corpus(root: &Path) -> anyhow::Result<BTreeMap<String, Result<Json, String>>> {
    let mut paths = Vec::new();
    collect(&root.join("plugins"), &mut paths)?;
    paths.sort();
    let mut out = BTreeMap::new();
    for (index, path) in paths.iter().enumerate() {
        let relative = path
            .strip_prefix(root)?
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        let source = std::fs::read_to_string(path)?;
        let projected = sylex_syl::parse(FileId(u32::try_from(index)?), &source)
            .map_err(|error| error.render(&relative, &source))
            .and_then(|file| {
                project(&file).map_err(|error| format!("{relative}: cannot project: {error}"))
            });
        out.insert(relative, projected);
    }
    Ok(out)
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "syl") {
            out.push(path);
        }
    }
    Ok(())
}

/// Compares `SylGen`'s dump with the projected `sylex_syl` trees, one
/// message per failing file. Every corpus file must parse, so a file either
/// side rejects (or `sylex_syl` parses but cannot project) fails, even
/// when both reject it.
#[must_use]
pub fn compare(
    lean: &BTreeMap<String, Json>,
    rust: &BTreeMap<String, Result<Json, String>>,
) -> Vec<String> {
    let mut differences = Vec::new();
    for path in lean.keys().filter(|path| !rust.contains_key(*path)) {
        differences.push(format!("{path}: only SylGen read this file"));
    }
    for (path, rust) in rust {
        let Some(lean) = lean.get(path) else {
            differences.push(format!("{path}: only sylex_syl read this file"));
            continue;
        };
        let lean_error = lean.get("error").and_then(Json::as_str);
        match (lean_error, rust) {
            (Some(lean), Err(rust)) => {
                differences.push(format!(
                    "{path}: both parsers reject it\n  SylGen:        {lean}\n  sylex_syl: {rust}"
                ));
            }
            (Some(lean), Ok(_)) => {
                differences.push(format!("{path}: only SylGen rejects it: {lean}"));
            }
            (None, Err(rust)) => {
                differences.push(format!("{path}: only sylex_syl rejects it: {rust}"));
            }
            (None, Ok(rust)) => {
                let lean =
                    normalize_integers(lean.get("declarations").cloned().unwrap_or(Json::Null));
                if let Some(mismatch) = first_mismatch(&lean, rust, "declarations") {
                    differences.push(format!("{path}: {mismatch}"));
                }
            }
        }
    }
    differences
}

/// Rewrites every `{"integer": text}` to carry the number, so integers compare
/// by value. Text that is not an `i64` stays text and so differs from any Rust
/// integer.
fn normalize_integers(json: Json) -> Json {
    match json {
        Json::Object(map) => {
            if let (1, Some(Json::String(text))) = (map.len(), map.get("integer"))
                && let Ok(number) = text.parse::<i64>()
            {
                return json!({ "integer": number });
            }
            Json::Object(
                map.into_iter()
                    .map(|(k, v)| (k, normalize_integers(v)))
                    .collect(),
            )
        }
        Json::Array(items) => Json::Array(items.into_iter().map(normalize_integers).collect()),
        other => other,
    }
}

/// The first place `lean` and `rust` differ, as its JSON path and both sides.
fn first_mismatch(lean: &Json, rust: &Json, path: &str) -> Option<String> {
    match (lean, rust) {
        (Json::Object(l), Json::Object(r)) => {
            let rust_only = r.keys().filter(|key| !l.contains_key(*key));
            for key in l.keys().chain(rust_only) {
                let here = format!("{path}.{key}");
                match (l.get(key), r.get(key)) {
                    (Some(l), Some(r)) => {
                        if let Some(found) = first_mismatch(l, r, &here) {
                            return Some(found);
                        }
                    }
                    (l, r) => return Some(show(&here, l, r)),
                }
            }
            None
        }
        (Json::Array(l), Json::Array(r)) => {
            for (index, (li, ri)) in l.iter().zip(r).enumerate() {
                if let Some(found) = first_mismatch(li, ri, &format!("{path}[{index}]")) {
                    return Some(found);
                }
            }
            (l.len() != r.len()).then(|| {
                let mut message = format!(
                    "{path}: SylGen has {} items, sylex_syl has {}",
                    l.len(),
                    r.len()
                );
                let extra = l.get(r.len()).map_or_else(
                    || format!("sylex_syl's next: {}", r[l.len()]),
                    |item| format!("SylGen's next: {item}"),
                );
                let _ = write!(message, "\n  {extra}");
                message
            })
        }
        _ => (lean != rust).then(|| show(path, Some(lean), Some(rust))),
    }
}

fn show(path: &str, lean: Option<&Json>, rust: Option<&Json>) -> String {
    let side = |j: Option<&Json>| j.map_or_else(|| "(absent)".to_owned(), Json::to_string);
    format!(
        "{path} differs\n  SylGen:        {}\n  sylex_syl: {}",
        side(lean),
        side(rust)
    )
}

/// Projects a parsed file onto the dump's shape: `SylGen`'s `Declaration`s,
/// without positions.
pub fn project(file: &File) -> Result<Json, String> {
    let mut out = Vec::new();
    for (index, item) in file.items.iter().enumerate() {
        let Some(declaration) = item.node() else {
            continue;
        };
        let kind = declaration.kind.ident.as_str();
        // `SylGen` ends every comment line with a newline, so a doc comment
        // ends with one.
        let mut documentation = String::new();
        for comment in file.doc_comment(index) {
            documentation.push_str(&comment.text);
            documentation.push('\n');
        }
        let implied_sort = json!({ "name": kind_sort(kind).unwrap_or(""), "args": [] });
        let projected = match &declaration.shape {
            Shape::Named(named) => json!({
                "kind": kind,
                "name": named.name.ident.as_str(),
                "parameters": named.params.iter().filter_map(|p| p.node()).map(|p| json!({
                    "name": p.name.ident.as_str(),
                    "sort": sort(&p.sort),
                    "default": p.default.as_ref().map(value),
                })).collect::<Vec<_>>(),
                "sort": named.sort.as_ref().map_or(implied_sort, sort),
                "fields": named.body.as_ref().map_or_else(Vec::new, |body| fields(body.nodes())),
                "constructor": named.constructor.map(|c| c.ident.as_str()),
                "cardValue": null,
                "documentation": documentation,
            }),
            Shape::Nameless(card_value) => json!({
                "kind": kind,
                "name": printed_name(card_value)?,
                "parameters": [],
                "sort": implied_sort,
                "fields": [],
                "constructor": null,
                "cardValue": value(card_value),
                "documentation": documentation,
            }),
        };
        out.push(projected);
    }
    Ok(Json::Array(out))
}

/// `SylGen`'s `kindSort`: the result sort a declaration kind implies.
fn kind_sort(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "action" => "Action",
        "ability" | "ability_word" => "Ability",
        "counter" => "CounterKind",
        "designation" => "DesignationLabel",
        "subtype" => "Subtype",
        "card_type" => "CardType",
        "relation" => "RelationLabel",
        "token" => "PredefinedToken",
        "sba" => "SbaRule",
        "conferral" => "ConferralRule",
        "card" => "Card",
        _ => return None,
    })
}

/// `SylGen`'s `printedName`: a card's name, its face names joined with `" //
/// "`.
fn printed_name(card: &Value) -> Result<String, String> {
    let record = match &card.kind {
        ValueKind::Record(record) => record,
        ValueKind::Call(call) => {
            let args: Vec<_> = call.args.iter().filter_map(|a| a.node()).collect();
            let [arg] = args.as_slice() else {
                return Err("expected named card parts".to_owned());
            };
            let (None, ValueKind::Record(record)) = (arg.name, &arg.value.kind) else {
                return Err("expected named card parts".to_owned());
            };
            let parts: &[&str] = match call.head.ident.as_str() {
                "transforming" | "modalDfc" => &["front", "back"],
                "split" | "sharedLineSplit" => &["left", "right"],
                "adventurer" => &["normal", "adventure"],
                "flip" => &["normal", "alternative"],
                "leveler" | "prototype" => &["inner"],
                _ => return Err("unknown card form".to_owned()),
            };
            let names = parts
                .iter()
                .map(|part| {
                    let field = record
                        .nodes()
                        .find(|f| f.name.ident == *part)
                        .ok_or_else(|| format!("missing card part '{part}'"))?;
                    printed_name(&field.value)
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(names.join(" // "));
        }
        _ => return Err("expected named card parts".to_owned()),
    };
    match record.nodes().find(|f| f.name.ident == "name") {
        Some(field) => match &field.value.kind {
            ValueKind::String(name) => Ok(name.clone()),
            _ => Err("card face requires a string name".to_owned()),
        },
        None => Err("card face requires a string name".to_owned()),
    }
}

fn sort(sort_ref: &Sort) -> Json {
    json!({
        "name": sort_ref.head.ident.as_str(),
        "args": sort_ref.args.iter().map(sort).collect::<Vec<_>>(),
    })
}

fn fields<'a>(fields: impl Iterator<Item = &'a sylex_syl::Field>) -> Vec<Json> {
    fields
        .map(|f| json!({ "name": f.name.ident.as_str(), "value": value(&f.value) }))
        .collect()
}

fn value(v: &Value) -> Json {
    match &v.kind {
        ValueKind::String(text) => json!({ "string": text }),
        ValueKind::Integer(number) => json!({ "integer": number }),
        ValueKind::List(items) => {
            json!({ "list": items.iter().filter_map(|i| i.node()).map(value).collect::<Vec<_>>() })
        }
        ValueKind::Record(record) => json!({ "record": fields(record.nodes()) }),
        ValueKind::Parameter(name) => json!({ "parameter": name.ident.as_str() }),
        ValueKind::Call(call) => json!({
            "word": call.head.ident.as_str(),
            "args": call.args.iter().filter_map(|a| a.node()).map(|a| json!({
                "name": a.name.map(|n| n.ident.as_str()),
                "value": value(&a.value),
            })).collect::<Vec<_>>(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "\
# Artifact.
#
# Second line.
card_type artifact { name = \"Artifact\", count = 3 }

# Detached.

primitive cost(x: Option(Mana) = none, y: Option(Mana) = $x): Cost = .mk
card split({ left = { name = \"Fire\" }, right = { name = \"Ice\" } })
";

    fn rust_side() -> BTreeMap<String, Result<Json, String>> {
        let file = sylex_syl::parse(FileId(0), SOURCE).unwrap();
        BTreeMap::from([("a.syl".to_owned(), project(&file))])
    }

    /// The dump `SylGen` writes for `SOURCE`.
    fn lean_side() -> BTreeMap<String, Json> {
        let declarations = json!([
            {
                "kind": "card_type", "name": "artifact", "parameters": [],
                "sort": { "name": "CardType", "args": [] },
                "fields": [
                    { "name": "name", "value": { "string": "Artifact" } },
                    { "name": "count", "value": { "integer": "3" } },
                ],
                "constructor": null, "cardValue": null,
                "documentation": " Artifact.\n\n Second line.\n",
            },
            {
                "kind": "primitive", "name": "cost",
                "parameters": [{
                    "name": "x",
                    "sort": { "name": "Option", "args": [{ "name": "Mana", "args": [] }] },
                    "default": { "word": "none", "args": [] },
                }, {
                    "name": "y",
                    "sort": { "name": "Option", "args": [{ "name": "Mana", "args": [] }] },
                    "default": { "parameter": "x" },
                }],
                "sort": { "name": "Cost", "args": [] },
                "fields": [], "constructor": "mk", "cardValue": null, "documentation": "",
            },
            {
                "kind": "card", "name": "Fire // Ice", "parameters": [],
                "sort": { "name": "Card", "args": [] }, "fields": [], "constructor": null,
                "cardValue": { "word": "split", "args": [{ "name": null, "value": { "record": [
                    { "name": "left", "value": { "record": [
                        { "name": "name", "value": { "string": "Fire" } }] } },
                    { "name": "right", "value": { "record": [
                        { "name": "name", "value": { "string": "Ice" } }] } },
                ] } }] },
                "documentation": "",
            },
        ]);
        BTreeMap::from([("a.syl".to_owned(), json!({ "declarations": declarations }))])
    }

    #[test]
    fn projection_matches_sylgen_shape() {
        assert_eq!(compare(&lean_side(), &rust_side()), Vec::<String>::new());
    }

    #[test]
    fn injected_mismatch_is_reported_with_path_and_both_sides() {
        let mut lean = lean_side();
        lean.get_mut("a.syl").unwrap()["declarations"][2]["cardValue"]["args"][0]["value"]["record"]
            [1]["value"]["record"][0]["value"]["string"] = json!("Water");
        let differences = compare(&lean, &rust_side());
        assert_eq!(differences.len(), 1, "{differences:?}");
        let message = &differences[0];
        assert!(message.starts_with("a.syl: "), "{message}");
        assert!(message.contains("\"Water\""), "{message}");
        assert!(message.contains("\"Ice\""), "{message}");
        assert!(message.contains("declarations[2]"), "{message}");
    }

    #[test]
    fn documentation_mismatch_is_reported() {
        let mut lean = lean_side();
        lean.get_mut("a.syl").unwrap()["declarations"][1]["documentation"] = json!(" Detached.\n");
        let differences = compare(&lean, &rust_side());
        assert!(
            differences[0].contains("declarations[1].documentation"),
            "{differences:?}"
        );
    }

    #[test]
    fn integers_compare_by_value_and_reject_other_text() {
        for text in ["4", "007", "9223372036854775808"] {
            let mut lean = lean_side();
            lean.get_mut("a.syl").unwrap()["declarations"][0]["fields"][1]["value"]["integer"] =
                json!(text);
            let differences = compare(&lean, &rust_side());
            assert_eq!(differences.len(), 1, "{text}: {differences:?}");
            assert!(differences[0].contains("fields[1]"), "{differences:?}");
        }
        let mut lean = lean_side();
        lean.get_mut("a.syl").unwrap()["declarations"][0]["fields"][1]["value"]["integer"] =
            json!("3");
        assert_eq!(compare(&lean, &rust_side()), Vec::<String>::new());
    }

    #[test]
    fn one_sided_files_and_errors_are_reported() {
        let mut lean = lean_side();
        lean.insert("b.syl".to_owned(), json!({ "error": "b.syl:1:1: bad" }));
        let mut rust = rust_side();
        rust.insert("c.syl".to_owned(), Err("c.syl:1:1: bad".to_owned()));
        let differences = compare(&lean, &rust);
        assert_eq!(differences.len(), 2, "{differences:?}");
        assert!(
            differences[0].contains("only SylGen read"),
            "{differences:?}"
        );
        assert!(
            differences[1].contains("only sylex_syl read"),
            "{differences:?}"
        );

        // A file both parsers reject still fails, with both errors.
        lean.insert("c.syl".to_owned(), json!({ "error": "lean says no" }));
        rust.insert("b.syl".to_owned(), Err("rust says no".to_owned()));
        let differences = compare(&lean, &rust);
        assert_eq!(differences.len(), 2, "{differences:?}");
        assert!(
            differences[0].starts_with("b.syl: both parsers reject"),
            "{differences:?}"
        );
        assert!(differences[0].contains("b.syl:1:1: bad"), "{differences:?}");
        assert!(differences[0].contains("rust says no"), "{differences:?}");

        lean.insert("c.syl".to_owned(), json!({ "declarations": [] }));
        assert!(compare(&lean, &rust)[1].contains("only sylex_syl rejects"));
    }

    #[test]
    fn projection_failure_is_reported() {
        let file = sylex_syl::parse(FileId(0), "card { types = [] }\n").unwrap();
        let projected = project(&file);
        assert!(projected.is_err(), "{projected:?}");
        let lean = BTreeMap::from([("d.syl".to_owned(), json!({ "declarations": [] }))]);
        let rust = BTreeMap::from([("d.syl".to_owned(), projected)]);
        let differences = compare(&lean, &rust);
        assert_eq!(differences.len(), 1, "{differences:?}");
        assert!(
            differences[0].contains("card face requires a string name"),
            "{differences:?}"
        );
    }
}
