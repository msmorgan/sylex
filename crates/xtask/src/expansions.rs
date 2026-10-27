//! Typed expansion snapshots of loose `plugins_v3` declarations.
//!
//! Each declaration is instantiated with deterministic constructor-basis
//! arguments. Defaulted parameters keep their defaults. Registry snapshots
//! include conferrals. Every declaration must produce a typed value; failures
//! make the command fail.
//!
//! Before a cleanup, save a baseline with
//! `cargo xtask expansions --out docs/evidence/plugins-v3-expansions/<ticket>/before`.
//! Make the change, then run
//! `cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/<ticket>/before`.
//! Comparison alone writes nothing. Add `--out <DIR>` to also save the new snapshot.
//! Use `--plugin plugins_v3/canon` on both runs to inspect only canon declarations,
//! expanded over the builtin prelude. The default plugin is `plugins_v3/builtin`.
//!
//! The versioned JSON `_index.txt` contains sorted records keyed by separate
//! `(bare_name, return_type)` fields, independent of source paths. Names (including
//! printed card names with punctuation or slashes) never become filesystem paths.
//! Use repeatable `--expect 'name : Type'` (or an unambiguous bare name) to accept
//! added, removed, or changed keys. Use `--rename 'old : Type=new : Type'` for
//! key renames: sampled arguments and expanded values must still agree, unless
//! the changed key is also expected. For names containing `=`, `--rename-file`
//! accepts a JSON array of `[old_key, new_key]` pairs. Unused allowances fail.
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Args;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::bundle::BundleKey;
use deckmaste_semantics_v3::lower::Elaborator;
use serde::Deserialize;
use serde::Serialize;

const INDEX_FILE: &str = "_index.txt";
const FORMAT_VERSION: u32 = 1;
type Outcomes = BTreeMap<BundleKey, Outcome>;

#[derive(Debug, Args)]
pub struct ExpansionsArgs {
    /// Bundle whose declarations to print, loaded over `plugins_v3/builtin`.
    #[arg(long, default_value_os_t = default_plugin_dir())]
    plugin: PathBuf,
    /// Output directory (default: target/expansions, or no writes with --compare).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Earlier keyed output directory to compare against, without modifying it.
    #[arg(long)]
    compare: Option<PathBuf>,
    /// Expected added, removed, or changed key: `name : Type` or bare `name`.
    #[arg(long, requires = "compare")]
    expect: Vec<String>,
    /// Key rename `OLD_KEY=NEW_KEY`; expanded values and arguments must agree.
    #[arg(long, requires = "compare")]
    rename: Vec<String>,
    /// JSON array of `[old_key, new_key]` pairs, also supporting names with `=`.
    #[arg(long, requires = "compare")]
    rename_file: Vec<PathBuf>,
    /// Load the bundle by itself, for example a saved copy of builtin.
    #[arg(long)]
    no_prelude: bool,
}
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn default_plugin_dir() -> PathBuf {
    workspace_root().join("plugins_v3/builtin")
}
fn default_out_dir() -> PathBuf {
    workspace_root().join("target/expansions")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Printed(String),
    Failed(String),
}

/// Produce one snapshot per declaration, with every failure retained.
#[must_use]
pub fn expand_all_keyed(bundle: &Bundle) -> BTreeMap<BundleKey, Outcome> {
    expand_keys(bundle, bundle.items.keys())
}

fn expand_keys<'a>(bundle: &Bundle, keys: impl Iterator<Item = &'a BundleKey>) -> Outcomes {
    let reader = Elaborator::new(bundle);
    keys.map(|key| {
        let result = reader
            .sample_arguments(key)
            .and_then(|(arguments, result)| {
                let value = reader.snapshot(key, &arguments, result)?;
                Ok(format!(
                    "// declaration: {} : {}\n// invocation: {} {arguments}\n{value}\n",
                    key.bare_name, key.return_type, key.bare_name
                ))
            });
        let outcome = match result {
            Ok(value) => Outcome::Printed(value),
            Err(error) => Outcome::Failed(error.to_string()),
        };
        (key.clone(), outcome)
    })
    .collect()
}

/// Produce snapshots indexed by source file. Prefer [`expand_all_keyed`] for comparisons.
#[must_use]
pub fn expand_all(bundle: &Bundle) -> BTreeMap<PathBuf, Outcome> {
    expand_all_keyed(bundle)
        .into_iter()
        .map(|(key, outcome)| (bundle.items[&key].file.clone(), outcome))
        .collect()
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    bare_name: String,
    return_type: String,
    outcome: Outcome,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    version: u32,
    declarations: Vec<Record>,
}
fn snapshot_bytes(outcomes: &Outcomes) -> anyhow::Result<Vec<u8>> {
    let snapshot = Snapshot {
        version: FORMAT_VERSION,
        declarations: outcomes
            .iter()
            .map(|(key, outcome)| Record {
                bare_name: key.bare_name.to_string(),
                return_type: key.return_type.to_string(),
                outcome: outcome.clone(),
            })
            .collect(),
    };
    let mut bytes = serde_json::to_vec_pretty(&snapshot)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn read_snapshot(directory: &Path) -> anyhow::Result<Outcomes> {
    let file = directory.join(INDEX_FILE);
    let snapshot: Snapshot = serde_json::from_slice(&std::fs::read(&file)?)
        .with_context(|| format!("reading keyed snapshot {}", file.display()))?;
    anyhow::ensure!(
        snapshot.version == FORMAT_VERSION,
        "unsupported snapshot version {}",
        snapshot.version
    );
    let mut outcomes = BTreeMap::new();
    for record in snapshot.declarations {
        let key = BundleKey {
            bare_name: record.bare_name.as_str().into(),
            return_type: record.return_type.as_str().into(),
        };
        anyhow::ensure!(
            matches!(record.outcome, Outcome::Printed(_)),
            "baseline contains a failed expansion: {}",
            key_label(&key)
        );
        anyhow::ensure!(
            outcomes.insert(key.clone(), record.outcome).is_none(),
            "duplicate baseline key: {}",
            key_label(&key)
        );
    }
    Ok(outcomes)
}
fn key_label(key: &BundleKey) -> String {
    format!("{} : {}", key.bare_name, key.return_type)
}
fn resolve_key(selector: &str, keys: &BTreeSet<BundleKey>) -> anyhow::Result<BundleKey> {
    let matches: Vec<_> = keys
        .iter()
        .filter(|key| selector == key_label(key) || selector == key.bare_name.as_str())
        .collect();
    anyhow::ensure!(
        matches.len() == 1,
        "key {selector:?} matches {} declarations; use an unambiguous key",
        matches.len()
    );
    Ok(matches[0].clone())
}

// Remove only the exact generated name headers, preserving arguments and the AST.
fn renamed_value<'a>(key: &BundleKey, outcome: &'a Outcome) -> anyhow::Result<&'a str> {
    let Outcome::Printed(text) = outcome else {
        anyhow::bail!("cannot compare failed expansion {}", key_label(key));
    };
    let header = format!(
        "// declaration: {}\n// invocation: {} ",
        key_label(key),
        key.bare_name
    );
    text.strip_prefix(&header)
        .with_context(|| format!("invalid expansion headers for {}", key_label(key)))
}

#[derive(Debug, PartialEq, Eq)]
enum Difference {
    Added(BundleKey),
    Removed(BundleKey),
    Changed(BundleKey),
}
impl Difference {
    fn key(&self) -> &BundleKey {
        match self {
            Self::Added(key) | Self::Removed(key) | Self::Changed(key) => key,
        }
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Added(_) => "added",
            Self::Removed(_) => "removed",
            Self::Changed(_) => "changed",
        }
    }
}
#[derive(Debug)]
struct Comparison {
    differences: Vec<Difference>,
    errors: Vec<String>,
}
impl Comparison {
    fn ensure_success(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.errors.is_empty(), "{}", self.errors.join("\n"));
        Ok(())
    }
}
fn compare(
    before: &Outcomes,
    after: &Outcomes,
    expected: &[String],
    renames: &[(String, String)],
) -> anyhow::Result<Comparison> {
    let old_keys: BTreeSet<_> = before.keys().cloned().collect();
    let new_keys: BTreeSet<_> = after.keys().cloned().collect();
    let keys: BTreeSet<_> = old_keys.union(&new_keys).cloned().collect();
    let mut renamed_old = BTreeSet::new();
    let mut renamed_new = BTreeSet::new();
    let mut accepted = BTreeSet::new();
    let mut differences = Vec::new();
    for (old, new) in renames {
        let old = resolve_key(old, &old_keys)?;
        let new = resolve_key(new, &new_keys)?;
        anyhow::ensure!(
            old != new && !new_keys.contains(&old) && !old_keys.contains(&new),
            "rename {} -> {} does not match a removed and added key",
            key_label(&old),
            key_label(&new)
        );
        anyhow::ensure!(
            renamed_old.insert(old.clone()) && renamed_new.insert(new.clone()),
            "duplicate rename endpoint"
        );
        differences.push(Difference::Removed(old.clone()));
        differences.push(Difference::Added(new.clone()));
        accepted.insert(old.clone());
        accepted.insert(new.clone());
        if renamed_value(&old, &before[&old])? != renamed_value(&new, &after[&new])? {
            differences.push(Difference::Changed(new.clone()));
            accepted.remove(&new);
        }
    }
    for key in &keys {
        if renamed_old.contains(key) || renamed_new.contains(key) {
            continue;
        }
        match (before.get(key), after.get(key)) {
            (None, Some(_)) => differences.push(Difference::Added(key.clone())),
            (Some(_), None) => differences.push(Difference::Removed(key.clone())),
            (Some(old), Some(new)) if old != new => {
                differences.push(Difference::Changed(key.clone()));
            }
            _ => {}
        }
    }
    differences.sort_by(|a, b| a.key().cmp(b.key()).then_with(|| a.label().cmp(b.label())));
    let mut errors = Vec::new();
    let mut expectations = BTreeSet::new();
    for selector in expected {
        let key = match resolve_key(selector, &keys) {
            Ok(key) => key,
            Err(error) => {
                errors.push(format!("unused expectation {selector:?}: {error}"));
                continue;
            }
        };
        let matched = differences.iter().any(|diff| {
            diff.key() == &key
                && (matches!(diff, Difference::Changed(_))
                    || (!renamed_old.contains(&key) && !renamed_new.contains(&key)))
        });
        if !matched || !expectations.insert(key.clone()) {
            errors.push(format!("unused expectation {selector:?}"));
        } else {
            accepted.insert(key);
        }
    }
    for diff in &differences {
        let mapped = match diff {
            Difference::Added(key) => renamed_new.contains(key),
            Difference::Removed(key) => renamed_old.contains(key),
            Difference::Changed(_) => false,
        };
        if !mapped && !accepted.contains(diff.key()) {
            errors.push(format!(
                "unexpected {} {}",
                diff.label(),
                key_label(diff.key())
            ));
        }
    }
    Ok(Comparison {
        differences,
        errors,
    })
}

// Canonicalize existing ancestors too, so symlink aliases cannot overwrite a baseline.
fn resolved_path(path: &Path) -> anyhow::Result<PathBuf> {
    if path.exists() {
        return Ok(path.canonicalize()?);
    }
    let absolute = std::path::absolute(path)?;
    let parent = absolute.parent().context("output has no parent")?;
    Ok(resolved_path(parent)?.join(absolute.file_name().context("output has no filename")?))
}

/// # Errors
/// Reports loading, expansion, comparison, or output errors. Nonempty unrelated
/// output directories and output paths overlapping the baseline are refused.
pub fn run(args: &ExpansionsArgs) -> anyhow::Result<()> {
    let baseline = args.compare.as_deref().map(read_snapshot).transpose()?;
    let output = args
        .out
        .clone()
        .or_else(|| args.compare.is_none().then(default_out_dir));
    if let (Some(before), Some(out)) = (&args.compare, &output) {
        let before = resolved_path(before)?;
        let out = resolved_path(out)?;
        anyhow::ensure!(
            !out.starts_with(&before) && !before.starts_with(&out),
            "output and baseline directories must not overlap"
        );
    }
    let root = args
        .plugin
        .canonicalize()
        .with_context(|| format!("resolving {}", args.plugin.display()))?;
    let local = Bundle::load(&root)?;
    let keys: BTreeSet<_> = local.items.keys().cloned().collect();
    let builtin = default_plugin_dir().canonicalize()?;
    let bundle = if args.no_prelude || root == builtin {
        local
    } else {
        let prelude = Bundle::load(&builtin)?;
        Bundle::from_items(
            prelude
                .items
                .into_values()
                .chain(local.items.into_values())
                .collect(),
        )?
    };
    let outcomes = expand_keys(&bundle, keys.iter());
    let failures: Vec<_> = outcomes
        .iter()
        .filter_map(|(key, outcome)| {
            if let Outcome::Failed(error) = outcome {
                Some(format!("{}: {error}", key_label(key)))
            } else {
                None
            }
        })
        .collect();
    println!(
        "{} declarations: {} printed, 0 skipped, {} failed",
        outcomes.len(),
        outcomes.len() - failures.len(),
        failures.len()
    );
    for failure in &failures {
        eprintln!("FAILED {failure}");
    }
    if let Some(out) = output {
        let bytes = snapshot_bytes(&outcomes)?;
        prepare_out_dir(&out)?;
        std::fs::write(out.join(INDEX_FILE), bytes)?;
        println!("written to {}", out.display());
    }
    anyhow::ensure!(
        failures.is_empty(),
        "{} declarations failed to expand",
        failures.len()
    );
    if let Some(before) = baseline {
        let mut renames = Vec::new();
        for rename in &args.rename {
            let (old, new) = rename
                .split_once('=')
                .context("rename requires OLD_KEY=NEW_KEY")?;
            renames.push((old.to_owned(), new.to_owned()));
        }
        for file in &args.rename_file {
            renames.extend(serde_json::from_slice::<Vec<(String, String)>>(
                &std::fs::read(file)?,
            )?);
        }
        let comparison = compare(&before, &outcomes, &args.expect, &renames)?;
        for diff in &comparison.differences {
            println!("{} {}", diff.label(), key_label(diff.key()));
        }
        println!("{} differences", comparison.differences.len());
        comparison.ensure_success()?;
    }
    Ok(())
}

fn prepare_out_dir(out: &Path) -> anyhow::Result<()> {
    if out.exists() {
        let empty = out.read_dir()?.next().is_none();
        anyhow::ensure!(
            empty || out.join(INDEX_FILE).is_file(),
            "{} is not empty and holds no `{INDEX_FILE}`; refusing to replace it",
            out.display()
        );
        std::fs::remove_dir_all(out).with_context(|| format!("clearing {}", out.display()))?;
    }
    std::fs::create_dir_all(out).with_context(|| format!("creating {}", out.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use deckmaste_semantics_v3::ast::Action;
    use deckmaste_semantics_v3::bundle::BundleItem;
    use deckmaste_semantics_v3::reader;

    use super::*;

    #[test]
    fn every_builtin_declaration_prints_without_skips() {
        let bundle = Bundle::load(&default_plugin_dir()).unwrap();
        let outcomes = expand_all(&bundle);
        assert_eq!(outcomes.len(), bundle.items.len());
        let failed: Vec<_> = outcomes
            .iter()
            .filter(|(_, v)| matches!(v, Outcome::Failed(_)))
            .collect();
        assert!(failed.is_empty(), "{failed:#?}");
    }

    #[test]
    fn a_helper_spelling_and_its_constructor_spelling_print_the_same() {
        let bundle = Bundle::load(&default_plugin_dir()).unwrap();
        let reader = Elaborator::new(&bundle);
        // Constructor basis checked against Lean's reduction of amass Zombie 3.
        let basis = r#"
            have creature : CardType := { name := "Creature", permanent := true, stats := [.power, .toughness] };
            have zombie : Subtype := .of creature "Zombie";
            have army : Predicate := .coord .and [
                .hasSubtype (.of creature "Army"), .hasType creature, .hasPossessor .controller .actor];
            have it : NounPhrase := .pro .bare .one;
            .ownScope (some (.action "amass" .none)) (.sequentially [
                .doIf (.not (.exists_ (.described (.a .unmarked) army)))
                    (.ownScope (some (.action "create" .none))
                        (.createObject (.inCaller (.lit 1))
                            (.token (.inCaller (.written { characteristics := {
                                colors := [.black], types := [creature], subtypes := [zombie, .of creature "Army"],
                                power := some (.lit 0), toughness := some (.lit 0) } })) []) none))
                    none false,
                .choose none (.described (.a .unmarked) army) .openly none,
                .putCounters (.inCaller (.lit __COUNT__)) (.printed (.named "p1p1Counter" .object))
                    (.pro (.word (.type creature)) .one),
                .doIf (.not (.matches it (.hasSubtype zombie)))
                    (.establish (.characteristicChange it [.typeLine .adds { subtypes := some [zombie] }]) none)
                    none false])
        "#;
        let helper = reader.read::<Action>("amass zombie 3").unwrap();
        let raw = reader
            .read::<Action>(&basis.replace("__COUNT__", "3"))
            .unwrap();
        assert_eq!(format!("{helper:#?}"), format!("{raw:#?}"));
        let changed = reader
            .read::<Action>(&basis.replace("__COUNT__", "4"))
            .unwrap();
        assert_ne!(format!("{helper:#?}"), format!("{changed:#?}"));
    }

    #[test]
    fn same_typed_params_get_distinct_samples() {
        let decl = reader::parse("primitive example (left right : Amount) : Amount := { body := .arith .plus left right }").unwrap();
        let bundle = Bundle::from_items(vec![BundleItem {
            declaration: decl,
            file: "example.lean".into(),
        }])
        .unwrap();
        let (arguments, _) = Elaborator::new(&bundle)
            .sample_arguments(&bundle.order[0])
            .unwrap();
        assert_eq!(arguments, "(left := 3) (right := 4)");
    }

    #[test]
    fn output_directory_requires_an_expansion_index() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output");
        std::fs::create_dir(&output).unwrap();
        std::fs::write(output.join("keep"), "original").unwrap();
        assert!(prepare_out_dir(&output).is_err());
        assert_eq!(
            std::fs::read_to_string(output.join("keep")).unwrap(),
            "original"
        );
        std::fs::write(output.join(INDEX_FILE), "").unwrap();
        prepare_out_dir(&output).unwrap();
        assert!(output.read_dir().unwrap().next().is_none());
    }

    #[test]
    fn malformed_expansions_are_reported_as_failures() {
        let declaration = reader::parse("template broken : Action := { body := missing }").unwrap();
        let bundle = Bundle::from_items(vec![BundleItem {
            declaration,
            file: "broken.lean".into(),
        }])
        .unwrap();
        assert!(matches!(
            expand_all(&bundle).values().next().unwrap(),
            Outcome::Failed(_)
        ));
    }
    fn fixture(name: &str, body: &str) -> Bundle {
        Bundle::from_items(vec![BundleItem {
            declaration: reader::parse(&format!("primitive {name} : Nat := {{ body := {body} }}"))
                .unwrap(),
            file: "original.lean".into(),
        }])
        .unwrap()
    }

    fn args(plugin: &Path, out: Option<&Path>, baseline: Option<&Path>) -> ExpansionsArgs {
        ExpansionsArgs {
            plugin: plugin.to_owned(),
            out: out.map(Path::to_owned),
            compare: baseline.map(Path::to_owned),
            expect: Vec::new(),
            rename: Vec::new(),
            rename_file: Vec::new(),
            no_prelude: true,
        }
    }

    #[test]
    fn keyed_dumps_are_byte_identical() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("plugin");
        std::fs::create_dir(&plugin).unwrap();
        std::fs::write(plugin.join("z.lean"), "primitive a : Nat := { body := 3 }").unwrap();
        std::fs::write(plugin.join("a.lean"), "primitive z : Nat := { body := 4 }").unwrap();
        let first = temp.path().join("first");
        let second = temp.path().join("second");
        run(&args(&plugin, Some(&first), None)).unwrap();
        run(&args(&plugin, Some(&second), None)).unwrap();
        assert_eq!(
            std::fs::read(first.join(INDEX_FILE)).unwrap(),
            std::fs::read(second.join(INDEX_FILE)).unwrap()
        );
        let snapshot = read_snapshot(&first).unwrap();
        assert_eq!(
            snapshot.keys().map(key_label).collect::<Vec<_>>(),
            ["a : Nat", "z : Nat"]
        );
    }

    #[test]
    fn moving_a_lean_file_reports_no_changes_and_compare_writes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("plugin");
        std::fs::create_dir(&plugin).unwrap();
        std::fs::write(
            plugin.join("before.lean"),
            "primitive value : Nat := { body := 3 }",
        )
        .unwrap();
        let baseline = temp.path().join("baseline");
        run(&args(&plugin, Some(&baseline), None)).unwrap();
        let original = std::fs::read(baseline.join(INDEX_FILE)).unwrap();
        let default_before = std::fs::read(default_out_dir().join(INDEX_FILE)).ok();
        std::fs::create_dir(plugin.join("nested")).unwrap();
        std::fs::rename(plugin.join("before.lean"), plugin.join("nested/after.lean")).unwrap();
        let after = expand_all_keyed(&Bundle::load(&plugin).unwrap());
        let comparison = compare(&read_snapshot(&baseline).unwrap(), &after, &[], &[]).unwrap();
        assert_eq!(comparison.differences, []);
        comparison.ensure_success().unwrap();
        run(&args(&plugin, None, Some(&baseline))).unwrap();
        assert_eq!(std::fs::read(baseline.join(INDEX_FILE)).unwrap(), original);
        assert_eq!(
            std::fs::read(default_out_dir().join(INDEX_FILE)).ok(),
            default_before
        );
        assert_eq!(baseline.read_dir().unwrap().count(), 1);
    }

    #[test]
    fn editing_a_body_requires_an_expectation() {
        let before = expand_all_keyed(&fixture("value", "3"));
        let after = expand_all_keyed(&fixture("value", "4"));
        let comparison = compare(&before, &after, &[], &[]).unwrap();
        assert_eq!(
            comparison.differences,
            [Difference::Changed(before.keys().next().unwrap().clone())]
        );
        assert!(comparison.ensure_success().is_err());
        for selector in ["value", "value : Nat"] {
            compare(&before, &after, &[selector.into()], &[])
                .unwrap()
                .ensure_success()
                .unwrap();
        }
    }

    #[test]
    fn declaration_renames_require_a_mapping_and_preserved_value() {
        let before = expand_all_keyed(&fixture("old", "3"));
        let after = expand_all_keyed(&fixture("new", "3"));
        let unmapped = compare(&before, &after, &[], &[]).unwrap();
        assert!(unmapped.ensure_success().is_err());
        assert!(matches!(
            &unmapped.differences[..],
            [Difference::Added(_), Difference::Removed(_)]
        ));
        let renames = [("old : Nat".into(), "new : Nat".into())];
        compare(&before, &after, &[], &renames)
            .unwrap()
            .ensure_success()
            .unwrap();
        let changed = expand_all_keyed(&fixture("new", "4"));
        assert!(
            compare(&before, &changed, &[], &renames)
                .unwrap()
                .ensure_success()
                .is_err()
        );
        compare(&before, &changed, &["new : Nat".into()], &renames)
            .unwrap()
            .ensure_success()
            .unwrap();
    }

    #[test]
    fn rename_comparison_preserves_invocation_arguments() {
        let make = |name: &str, parameter: &str| {
            Bundle::from_items(vec![BundleItem {
                declaration: reader::parse(&format!(
                    "primitive {name} ({parameter} : Nat) : Nat := {{ body := 3 }}"
                ))
                .unwrap(),
                file: "example.lean".into(),
            }])
            .unwrap()
        };
        let before = expand_all_keyed(&make("old", "count"));
        let after = expand_all_keyed(&make("new", "count"));
        let renames = [("old".into(), "new".into())];
        compare(&before, &after, &[], &renames)
            .unwrap()
            .ensure_success()
            .unwrap();
        let changed = expand_all_keyed(&make("new", "amount"));
        assert!(
            compare(&before, &changed, &[], &renames)
                .unwrap()
                .ensure_success()
                .is_err()
        );
    }

    #[test]
    fn arbitrary_names_and_overloaded_keys_round_trip_without_paths() {
        let names = [
            "Branchloft Pathway // Boulderloft Pathway",
            "../Circle of Protection: \"Red\" / Dr. X = Y",
            "with : delimiter : Spelled",
            "line\nbreak\\and\ttab",
        ];
        let mut items = Vec::new();
        for name in names {
            for source in [
                "primitive placeholder : Nat := { body := 3 }",
                "primitive placeholder : Bool := { body := false }",
            ] {
                let mut declaration = reader::parse(source).unwrap();
                declaration.name = name.into();
                items.push(BundleItem {
                    declaration,
                    file: "shared.lean".into(),
                });
            }
        }
        let bundle = Bundle::from_items(items).unwrap();
        let outcomes = expand_all_keyed(&bundle);
        assert_eq!(outcomes.len(), names.len() * 2);
        assert!(
            outcomes
                .values()
                .all(|outcome| matches!(outcome, Outcome::Printed(_)))
        );
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join(INDEX_FILE),
            snapshot_bytes(&outcomes).unwrap(),
        )
        .unwrap();
        assert_eq!(read_snapshot(temp.path()).unwrap(), outcomes);
        assert_eq!(temp.path().read_dir().unwrap().count(), 1);
        assert_eq!(
            snapshot_bytes(&outcomes).unwrap(),
            snapshot_bytes(&expand_all_keyed(&bundle)).unwrap()
        );
        compare(&outcomes, &outcomes, &[], &[])
            .unwrap()
            .ensure_success()
            .unwrap();
    }

    #[test]
    fn additions_and_removals_are_sorted_and_require_expectations() {
        let before = expand_all_keyed(&fixture("z", "3"));
        let after = expand_all_keyed(&fixture("a", "4"));
        let result = compare(&before, &after, &[], &[]).unwrap();
        assert_eq!(
            result
                .differences
                .iter()
                .map(|diff| format!("{} {}", diff.label(), key_label(diff.key())))
                .collect::<Vec<_>>(),
            ["added a : Nat", "removed z : Nat"]
        );
        assert!(result.ensure_success().is_err());
        compare(&before, &after, &["z".into(), "a".into()], &[])
            .unwrap()
            .ensure_success()
            .unwrap();
    }

    #[test]
    fn unused_and_ambiguous_allowances_fail() {
        let before = expand_all_keyed(&fixture("value", "3"));
        for expected in [vec!["value".into()], vec!["missing".into()]] {
            assert!(
                compare(&before, &before, &expected, &[])
                    .unwrap()
                    .ensure_success()
                    .is_err()
            );
        }
        assert!(compare(&before, &before, &[], &[("value".into(), "value".into())]).is_err());
        assert!(compare(&before, &before, &[], &[("missing".into(), "value".into())]).is_err());
        let after = expand_all_keyed(&fixture("value", "4"));
        assert!(
            compare(
                &before,
                &after,
                &["value".into(), "value : Nat".into()],
                &[]
            )
            .unwrap()
            .ensure_success()
            .is_err()
        );
        let mut overloaded = before.clone();
        let key = BundleKey {
            bare_name: "value".into(),
            return_type: "Bool".into(),
        };
        overloaded.insert(key, Outcome::Printed("false".into()));
        assert!(
            compare(&overloaded, &overloaded, &["value".into()], &[])
                .unwrap()
                .ensure_success()
                .is_err()
        );
    }

    #[test]
    fn duplicate_rename_endpoints_and_redundant_expectations_fail() {
        let before = expand_all_keyed(&fixture("old", "3"));
        let after = expand_all_keyed(&fixture("new", "3"));
        let rename = ("old".into(), "new".into());
        assert!(compare(&before, &after, &[], &[rename.clone(), rename.clone()]).is_err());
        assert!(
            compare(&before, &after, &["new".into()], &[rename])
                .unwrap()
                .ensure_success()
                .is_err()
        );
    }

    #[test]
    fn cli_compares_expected_edits_and_renames_and_can_save_new_output() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("plugin");
        std::fs::create_dir(&plugin).unwrap();
        let source = plugin.join("example.lean");
        std::fs::write(&source, "primitive old : Nat := { body := 3 }").unwrap();
        let baseline = temp.path().join("baseline");
        run(&args(&plugin, Some(&baseline), None)).unwrap();
        let original = std::fs::read(baseline.join(INDEX_FILE)).unwrap();
        std::fs::write(&source, "primitive old : Nat := { body := 4 }").unwrap();
        let mut comparison = args(&plugin, None, Some(&baseline));
        assert!(run(&comparison).is_err());
        comparison.expect.push("old : Nat".into());
        run(&comparison).unwrap();
        std::fs::write(&source, "primitive new : Nat := { body := 3 }").unwrap();
        comparison.expect.clear();
        assert!(run(&comparison).is_err());
        comparison.rename.push("old : Nat=new : Nat".into());
        let output = temp.path().join("after");
        comparison.out = Some(output.clone());
        run(&comparison).unwrap();
        assert_eq!(
            read_snapshot(&output).unwrap(),
            expand_all_keyed(&fixture("new", "3"))
        );
        let rename_file = temp.path().join("renames.json");
        std::fs::write(&rename_file, r#"[["old : Nat", "new : Nat"]]"#).unwrap();
        comparison.rename.clear();
        comparison.rename_file.push(rename_file);
        run(&comparison).unwrap();
        std::fs::write(&source, "primitive new : Nat := { body := 4 }").unwrap();
        assert!(run(&comparison).is_err());
        comparison.expect.push("new".into());
        run(&comparison).unwrap();
        assert_eq!(std::fs::read(baseline.join(INDEX_FILE)).unwrap(), original);
    }

    #[test]
    fn comparison_rejects_outputs_overlapping_the_baseline() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("plugin");
        std::fs::create_dir(&plugin).unwrap();
        std::fs::write(
            plugin.join("example.lean"),
            "primitive value : Nat := { body := 3 }",
        )
        .unwrap();
        let baseline = temp.path().join("baseline");
        run(&args(&plugin, Some(&baseline), None)).unwrap();
        let original = std::fs::read(baseline.join(INDEX_FILE)).unwrap();
        for out in [&baseline, &baseline.join("child"), temp.path()] {
            assert!(run(&args(&plugin, Some(out), Some(&baseline))).is_err());
            assert_eq!(std::fs::read(baseline.join(INDEX_FILE)).unwrap(), original);
        }
    }

    #[cfg(unix)]
    #[test]
    fn comparison_rejects_a_symlink_alias_of_the_baseline() {
        let temp = tempfile::tempdir().unwrap();
        let baseline = temp.path().join("baseline");
        std::fs::create_dir(&baseline).unwrap();
        std::fs::write(
            baseline.join(INDEX_FILE),
            snapshot_bytes(&expand_all_keyed(&fixture("value", "3"))).unwrap(),
        )
        .unwrap();
        let alias = temp.path().join("alias");
        std::os::unix::fs::symlink(&baseline, &alias).unwrap();
        assert!(run(&args(temp.path(), Some(&alias), Some(&baseline))).is_err());
        assert!(read_snapshot(&baseline).is_ok());
    }

    #[test]
    fn malformed_or_failed_baselines_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join(INDEX_FILE);
        std::fs::write(&file, "old path-based index").unwrap();
        assert!(read_snapshot(temp.path()).is_err());
        let good = snapshot_bytes(&expand_all_keyed(&fixture("value", "3"))).unwrap();
        let mut snapshot: Snapshot = serde_json::from_slice(&good).unwrap();
        snapshot.version += 1;
        std::fs::write(&file, serde_json::to_vec(&snapshot).unwrap()).unwrap();
        assert!(read_snapshot(temp.path()).is_err());
        snapshot.version = FORMAT_VERSION;
        snapshot.declarations[0].outcome = Outcome::Failed("broken".into());
        std::fs::write(&file, serde_json::to_vec(&snapshot).unwrap()).unwrap();
        assert!(read_snapshot(temp.path()).is_err());
        let mut duplicate: serde_json::Value = serde_json::from_slice(&good).unwrap();
        let declarations = duplicate["declarations"].as_array_mut().unwrap();
        declarations.push(declarations[0].clone());
        std::fs::write(&file, serde_json::to_vec(&duplicate).unwrap()).unwrap();
        assert!(read_snapshot(temp.path()).is_err());
    }
}
