//! Typed expansion snapshots of loose `plugins_v3` declarations.
//!
//! Each declaration is instantiated with deterministic constructor-basis
//! arguments. Defaulted parameters keep their defaults. Registry snapshots
//! include conferrals. Every declaration must produce a typed value; failures
//! make the command fail.
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Args;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::lower::Elaborator;

const INDEX_FILE: &str = "_index.txt";
const SKIPPED_FILE: &str = "_skipped.txt";

#[derive(Debug, Args)]
pub struct ExpansionsArgs {
    /// Bundle whose declarations to print, loaded over `plugins_v3/builtin`.
    #[arg(long, default_value_os_t = default_plugin_dir())]
    plugin: PathBuf,
    /// Output directory: absent, empty, or an earlier output of this command.
    #[arg(long, default_value_os_t = default_out_dir())]
    out: PathBuf,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Printed(String),
    Failed(String),
}

/// Produce one snapshot per declaration, with every failure retained.
#[must_use]
pub fn expand_all(bundle: &Bundle) -> BTreeMap<PathBuf, Outcome> {
    let reader = Elaborator::new(bundle);
    bundle
        .order
        .iter()
        .map(|key| {
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
            (bundle.items[key].file.clone(), outcome)
        })
        .collect()
}

/// # Errors
/// Reports loading, expansion, or output errors. Nonempty unrelated output
/// directories are refused.
pub fn run(args: &ExpansionsArgs) -> anyhow::Result<()> {
    let root = args
        .plugin
        .canonicalize()
        .with_context(|| format!("resolving {}", args.plugin.display()))?;
    let local = Bundle::load(&root)?;
    let files: BTreeSet<_> = local.items.values().map(|i| i.file.clone()).collect();
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
    let outcomes: BTreeMap<_, _> = expand_all(&bundle)
        .into_iter()
        .filter(|(path, _)| files.contains(path))
        .collect();
    prepare_out_dir(&args.out)?;
    let mut index = String::new();
    let mut failures = Vec::new();
    let mut printed = 0;
    for (path, outcome) in &outcomes {
        let relative = path
            .strip_prefix(&root)
            .with_context(|| format!("{} is outside {}", path.display(), root.display()))?;
        match outcome {
            Outcome::Printed(text) => {
                let file = args.out.join(relative).with_extension("expanded");
                if let Some(parent) = file.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&file, text)
                    .with_context(|| format!("writing {}", file.display()))?;
                writeln!(index, "printed  {}", relative.display())?;
                printed += 1;
            }
            Outcome::Failed(error) => {
                writeln!(index, "FAILED   {}", relative.display())?;
                failures.push(format!("{}: {error}", relative.display()));
            }
        }
    }
    let summary = format!(
        "{} declarations: {printed} printed, 0 skipped, {} failed",
        outcomes.len(),
        failures.len()
    );
    writeln!(index, "{summary}")?;
    std::fs::write(args.out.join(INDEX_FILE), index)?;
    std::fs::write(args.out.join(SKIPPED_FILE), "")?;
    println!("{summary}\nwritten to {}", args.out.display());
    for failure in &failures {
        eprintln!("FAILED {failure}");
    }
    anyhow::ensure!(
        failures.is_empty(),
        "{} declarations failed to expand",
        failures.len()
    );
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
}
