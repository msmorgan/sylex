//! `cargo xtask lean check`: build the native Lean workbench, then rerun the
//! Lake generation regression against its lakefile.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use anyhow::Context as _;
use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
pub struct LeanArgs {
    #[command(subcommand)]
    command: LeanCmd,
}

#[derive(Debug, Subcommand)]
enum LeanCmd {
    /// Build every library (syntax, checker, builtin, card bench, proofs) with
    /// warnings as errors, then check that Lake regenerates the canon card
    /// modules.
    Check,
}

/// Every library in `lean/lakefile.lean`. The default target is only the fast
/// loop, so the gate names them all.
const LIBRARIES: [&str; 8] = [
    "Semantics",
    "SemanticsDefinitions",
    "SemanticsCheckBuiltin",
    "SemanticsCardsData",
    "SemanticsCards",
    "SemanticsProofs",
    "SylFixtures",
    "SylFixturePins",
];

pub fn run(args: &LeanArgs) -> anyhow::Result<()> {
    match args.command {
        LeanCmd::Check => {
            let workbench = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lean");
            check(&mut Command::new("lake"), &workbench)?;
            generation::regression(&|| Command::new("lake"), &workbench)
        }
    }
}

fn check(lake: &mut Command, workbench: &Path) -> anyhow::Result<()> {
    let status = lake
        .current_dir(workbench)
        .args(["build", "-q", "--wfail"])
        .args(LIBRARIES)
        .status()
        .context("starting the Lean workbench build")?;
    anyhow::ensure!(status.success(), "Lean workbench build failed: {status}");
    Ok(())
}

/// Exercise the production generator and Lake facet overrides in a disposable
/// package.
mod generation {
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    use super::*;

    pub fn regression(lake: &dyn Fn() -> Command, workbench: &Path) -> anyhow::Result<()> {
        let scratch = tempfile::tempdir().context("creating the Lake regression package")?;
        let package = scratch.path();
        let workbench = workbench.canonicalize()?;
        let source = std::fs::read_to_string(workbench.join("lakefile.lean"))?;
        let helper = section(&source, "def writeIfChanged", "input_dir pluginCorpus")?;
        let input_facet = section(&source, "@[«module_facet»]", "/-! ## Libraries")?;
        let modules_facet = section(&source, "@[«library_facet»]", "lean_lib SylFixtures")?;
        let generator = workbench.join(".lake/build/bin/sylGen");
        let lean_path = workbench.join(".lake/build/lib/lean");
        let config = format!(
            r#"import Lake
open Lake DSL System
package regression
require mtg from "{workbench}"
{helper}
{input_facet}
{modules_facet}
target genRegression pkg : Unit := Job.async do
  let result ← IO.Process.output {{
    cmd := "{generator}"
    env := #[("LEAN_PATH", some "{lean_path}")]
    args := #[(pkg.dir / "input").toString, (pkg.dir / ".lake/build/gen").toString,
      "Regression", "Bench"]
  }}
  unless result.exitCode == 0 do error (result.stdout ++ result.stderr)
  for entry in ← (pkg.dir / ".lake/build/gen/Bench").readDir do
    if entry.path.extension == some "lean" then
      if (← IO.FS.readFile entry.path).contains "«Fixture Card»" then
        writeIfChanged (pkg.dir / ".lake/build/gen/FixtureOnly.lean")
          s!"import Bench.{{entry.path.fileStem.getD ""}}\n"
  setTrace .nil
lean_lib Bench where
  srcDir := ".lake/build/gen"
  roots := #[`Bench, `FixtureOnly]
  needs := #[genRegression]
lean_lib Proof where
  roots := #[`Proof]
"#,
            workbench = workbench.display(),
            generator = generator.display(),
            lean_path = lean_path.display()
        );
        std::fs::write(package.join("lakefile.lean"), config)?;
        std::fs::copy(
            workbench.join("lean-toolchain"),
            package.join("lean-toolchain"),
        )?;
        let input = package.join("input");
        std::fs::create_dir_all(&input)?;
        std::fs::write(
            input.join("artifact.syl"),
            "card_type artifact { name = \"artifact\", cardClass = permanentCard }\n",
        )?;
        write_card(&input, "Alpha", 1)?;
        write_card(&input, "Fixture Card", 1)?;
        std::fs::write(
            package.join("Proof.lean"),
            concat!(
                "import FixtureOnly\nimport Semantics.Check.Card\n",
                "example : Semantics.Card.check Regression.Semantics.Cards.«Fixture Card» = [] := ",
                "by decide\n"
            ),
        )?;
        let built = build(lake, package)?;
        let alpha = card_module(package, "Alpha")?;
        let fixture = card_module(package, "Fixture Card")?;
        for module in [&alpha, &fixture] {
            anyhow::ensure!(
                built.contains(&format!("Bench.{module}")),
                "missing data module {module}"
            );
            anyhow::ensure!(
                built.contains(&format!("Bench.Check.{module}")),
                "missing check module {module}"
            );
        }
        anyhow::ensure!(built.contains("Proof"), "clean build omitted the proof");
        anyhow::ensure!(
            build(lake, package)?.is_empty(),
            "no-op build rebuilt modules"
        );
        write_card(&input, "Alpha", 2)?;
        let mut built = build(lake, package)?;
        built.remove("Bench");
        let expected = BTreeSet::from([format!("Bench.{alpha}"), format!("Bench.Check.{alpha}")]);
        anyhow::ensure!(
            built == expected,
            "card edit rebuilt {built:?}, expected {expected:?}"
        );
        write_card(&input, "Fixture Card", 2)?;
        let mut built = build(lake, package)?;
        built.remove("Bench");
        let expected = BTreeSet::from([
            format!("Bench.{fixture}"),
            format!("Bench.Check.{fixture}"),
            "FixtureOnly".to_owned(),
            "Proof".to_owned(),
        ]);
        anyhow::ensure!(
            built == expected,
            "fixture edit rebuilt {built:?}, expected {expected:?}"
        );
        let generated = package.join(format!(".lake/build/gen/Bench/{fixture}.lean"));
        let previous = std::fs::read_to_string(&generated)?;
        write_card(&input, "Fixture Card", 3)?;
        let output = lake()
            .current_dir(package)
            .args(["setup-file", "Proof.lean"])
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "setup-file failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        anyhow::ensure!(
            std::fs::read_to_string(&generated)? != previous,
            "setup-file read stale generated card source"
        );
        changing_module_set(lake, package)?;
        cached_generation(lake, &workbench)
    }

    fn cached_generation(lake: &dyn Fn() -> Command, workbench: &Path) -> anyhow::Result<()> {
        let stamps = ["syl.stamp", "plugins.stamp", "proof-fixtures.stamp"];
        let before = stamps.map(|name| {
            std::fs::metadata(workbench.join(".lake/build/gen").join(name))
                .and_then(|m| m.modified())
        });
        let output = lake()
            .current_dir(workbench)
            .args([
                "build",
                "-q",
                "SylFixtures",
                "SylFixturePins",
                "SemanticsCardsData",
            ])
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "cached generation build failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        for (name, previous) in stamps.into_iter().zip(before) {
            anyhow::ensure!(
                std::fs::metadata(workbench.join(".lake/build/gen").join(name))?.modified()?
                    == previous?,
                "unchanged build reran {name}"
            );
        }
        Ok(())
    }

    fn section<'a>(source: &'a str, start: &str, end: &str) -> anyhow::Result<&'a str> {
        let (_, rest) = source
            .split_once(start)
            .context("missing production Lake definition")?;
        let (body, _) = rest
            .split_once(end)
            .context("missing production Lake boundary")?;
        let offset = source.len() - rest.len() - start.len();
        Ok(&source[offset..offset + start.len() + body.len()])
    }

    fn write_card(input: &Path, name: &str, cost: u32) -> anyhow::Result<()> {
        std::fs::write(
            input.join(format!("{name}.syl")),
            format!("card {{ name = \"{name}\", cost = [{cost}], types = [artifact] }}\n"),
        )?;
        Ok(())
    }

    fn card_module(package: &Path, name: &str) -> anyhow::Result<String> {
        for entry in std::fs::read_dir(package.join(".lake/build/gen/Bench"))? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "lean")
                && std::fs::read_to_string(&path)?.contains(&format!("«{name}»"))
            {
                return Ok(path
                    .file_stem()
                    .context("module stem")?
                    .to_string_lossy()
                    .into_owned());
            }
        }
        anyhow::bail!("generator omitted {name}")
    }

    fn artifacts(dir: &Path) -> anyhow::Result<BTreeMap<String, SystemTime>> {
        fn walk(
            root: &Path,
            dir: &Path,
            out: &mut BTreeMap<String, SystemTime>,
        ) -> anyhow::Result<()> {
            if !dir.exists() {
                return Ok(());
            }
            for entry in std::fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    walk(root, &path, out)?;
                } else if path.extension().is_some_and(|ext| ext == "olean") {
                    let module = path
                        .strip_prefix(root)?
                        .with_extension("")
                        .to_string_lossy()
                        .replace('/', ".");
                    out.insert(module, std::fs::metadata(path)?.modified()?);
                }
            }
            Ok(())
        }
        let mut result = BTreeMap::new();
        walk(dir, dir, &mut result)?;
        Ok(result)
    }

    fn build(lake: &dyn Fn() -> Command, package: &Path) -> anyhow::Result<BTreeSet<String>> {
        let dir = package.join(".lake/build/lib/lean");
        let before = artifacts(&dir)?;
        let output = lake()
            .current_dir(package)
            .args(["build", "-q", "Bench", "Proof"])
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "Lake regression build failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(artifacts(&dir)?
            .into_iter()
            .filter_map(|(name, time)| (before.get(&name) != Some(&time)).then_some(name))
            .collect())
    }
    fn changing_module_set(lake: &dyn Fn() -> Command, package: &Path) -> anyhow::Result<()> {
        // Exercise the production modules-facet override with a library whose
        // glob changes. The small generator isolates Lake ordering from
        // semantic compilation costs.
        let config = package.join("lakefile.lean");
        let mut source = std::fs::read_to_string(&config)?;
        source.push_str(
            r#"

target genChanging pkg : Unit := Job.async do
  let name := (← IO.FS.readFile (pkg.dir / "module-name")).trimAscii.toString
  let dir := pkg.dir / ".lake/build/gen/Changing"
  IO.FS.createDirAll dir
  for entry in ← dir.readDir do
    if entry.fileName != name ++ ".lean" then IO.FS.removeFile entry.path
  writeIfChanged (dir / (name ++ ".lean")) ("def " ++ name ++ " : Nat := 1\n")
  setTrace .nil

lean_lib Changing where
  srcDir := ".lake/build/gen"
  roots := #[`Changing]
  globs := #[.submodules `Changing]
  needs := #[genChanging]
"#,
        );
        std::fs::write(&config, source)?;
        for name in ["AddedFirst", "AddedSecond"] {
            std::fs::write(package.join("module-name"), name)?;
            let output = lake()
                .current_dir(package)
                .args(["build", "-q", "Changing"])
                .output()
                .context("building a changing generated-module set")?;
            anyhow::ensure!(
                output.status.success(),
                "changing generated-module set failed: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            anyhow::ensure!(
                package
                    .join(format!(".lake/build/lib/lean/Changing/{name}.olean"))
                    .exists(),
                "new generated module {name} was not built"
            );
        }
        anyhow::ensure!(
            !package
                .join(".lake/build/gen/Changing/AddedFirst.lean")
                .exists(),
            "removed generated module remained in the source set"
        );
        Ok(())
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn stub(body: &str) -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let script = dir.path().join("lake.sh");
        std::fs::write(&script, body)?;
        let mut command = Command::new("sh");
        command.arg(&script).env("EXPECTED_WORKBENCH", dir.path());
        check(&mut command, dir.path())
    }

    #[test]
    fn check_builds_every_library_with_warnings_as_errors() {
        stub(
            "test \"$PWD\" = \"$EXPECTED_WORKBENCH\" && test \"$*\" = \
             'build -q --wfail Semantics SemanticsDefinitions SemanticsCheckBuiltin \
             SemanticsCardsData SemanticsCards SemanticsProofs SylFixtures SylFixturePins'",
        )
        .unwrap();
    }

    #[test]
    fn check_propagates_build_failure() {
        let error = stub("exit 7").unwrap_err();
        assert!(error.to_string().contains("Lean workbench build failed"));
    }

    #[test]
    fn check_propagates_failure_to_start_lake() {
        let dir = tempfile::tempdir().unwrap();
        let error = check(
            &mut Command::new(dir.path().join("missing-lake")),
            dir.path(),
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("starting the Lean workbench build")
        );
    }
}
