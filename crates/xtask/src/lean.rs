//! `cargo xtask lean check`: build the native Lean workbench, then rerun the Lake generation
//! regression against its lakefile.

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
    /// Build every library (syntax, checker, builtin, card bench, proofs) with warnings as
    /// errors, then check that Lake regenerates the canon card modules.
    Check,
}

/// Every library in `lean/lakefile.lean`. The default target is only the fast loop, so the gate
/// names them all.
const LIBRARIES: [&str; 6] = [
    "Semantics",
    "SemanticsDefinitions",
    "SemanticsCheckBuiltin",
    "SemanticsCardsData",
    "SemanticsCards",
    "SemanticsProofs",
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

/// The regression for `genCanon` and the `input` facet override in `lean/lakefile.lean`, which
/// depend on Lake internals a toolchain bump could change. It copies the real lakefile and
/// `Semantics/Cards.lean` into a throwaway package whose card bench is the stub tree
/// `lean/test-fixtures/lake-generation/` (`card := <n>` files, and stub `canon_card` and
/// `check_canon_card` commands), and drives Lake through the edits the ticket
/// `lean-generated-canon-modules` names.
mod generation {
    use super::*;

    const SHARD: &str = "Alpha.lean";
    const FIXTURE: &str = "Fixture Card.lean";
    const PROOF: &str = "Semantics/Proofs/Uses.lean";
    /// Pure re-exports above the edited modules. They rebuild whenever an imported `.olean`
    /// changes, which says nothing about what was regenerated.
    const UMBRELLAS: [&str; 2] = ["Semantics.Cards", "Semantics.Proofs"];

    pub fn regression(lake: &dyn Fn() -> Command, workbench: &Path) -> anyhow::Result<()> {
        let scratch = tempfile::tempdir().context("creating the Lake regression package")?;
        let root = scratch.path();
        copy_dir(&workbench.join("test-fixtures/lake-generation"), root)?;
        let package = root.join("lean");
        for file in ["lakefile.lean", "lean-toolchain", "Semantics/Cards.lean"] {
            std::fs::copy(workbench.join(file), package.join(file))
                .with_context(|| format!("copying {file} into the Lake regression package"))?;
        }
        std::fs::create_dir_all(root.join("plugins_v3/builtin"))?;
        let cards = root.join("plugins_v3/canon/cards");
        let generated = package.join("Semantics/Cards");

        // A clean tree, with no generated module present, builds.
        let built = build(lake, &package)?;
        for module in generated_modules() {
            anyhow::ensure!(
                built.contains(&module),
                "a clean build did not build {module}: {built:?}"
            );
        }
        let rebuilt = build(lake, &package)?;
        anyhow::ensure!(rebuilt.is_empty(), "a no-op build rebuilt {rebuilt:?}");

        // Editing one card rebuilds exactly its data shard and check shard.
        let shard = shard_of(&generated, SHARD)?;
        std::fs::write(cards.join(SHARD), "card := 3\n")?;
        let built = without_umbrellas(build(lake, &package)?);
        let expected = BTreeSet::from([
            format!("Semantics.Cards.Data.{shard}"),
            format!("Semantics.Cards.Check.{shard}"),
        ]);
        anyhow::ensure!(
            built == expected,
            "editing {SHARD} rebuilt {built:?}, not {expected:?}"
        );

        // Editing a fixture card rebuilds `Fixtures` and the proof that imports it.
        std::fs::write(cards.join(FIXTURE), "card := 0 + 1\n")?;
        let built = without_umbrellas(build(lake, &package)?);
        let expected = BTreeSet::from([
            "Semantics.Cards.Data.Fixtures".to_owned(),
            "Semantics.Cards.Check.Fixtures".to_owned(),
            "Semantics.Proofs.Uses".to_owned(),
        ]);
        anyhow::ensure!(
            built == expected,
            "editing {FIXTURE} rebuilt {built:?}, not {expected:?}"
        );

        // The language server's `lake setup-file` on a proof regenerates after a card edit.
        std::fs::write(cards.join(FIXTURE), "card := 1 * 1\n")?;
        let output = lake()
            .current_dir(&package)
            .args(["setup-file", PROOF])
            .output()
            .context("starting lake setup-file")?;
        anyhow::ensure!(
            output.status.success(),
            "lake setup-file {PROOF} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let fixtures = std::fs::read_to_string(generated.join("Data/Fixtures.lean"))?;
        anyhow::ensure!(
            fixtures.contains("card := 1 * 1"),
            "lake setup-file {PROOF} did not regenerate Semantics/Cards/Data/Fixtures.lean"
        );
        Ok(())
    }

    /// The modules Lake built, from its `Built <module> (<time>)` lines.
    fn build(lake: &dyn Fn() -> Command, package: &Path) -> anyhow::Result<BTreeSet<String>> {
        let output = lake()
            .current_dir(package)
            .args(["build", "SemanticsCards", "SemanticsProofs"])
            .output()
            .context("starting the Lake regression build")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::ensure!(
            output.status.success(),
            "the Lake regression build failed:\n{stdout}{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(built_modules(&stdout))
    }

    pub(super) fn built_modules(log: &str) -> BTreeSet<String> {
        log.lines()
            .filter_map(|line| line.split_once("Built ")?.1.split_whitespace().next())
            .map(str::to_owned)
            .collect()
    }

    fn without_umbrellas(mut built: BTreeSet<String>) -> BTreeSet<String> {
        for umbrella in UMBRELLAS {
            built.remove(umbrella);
        }
        built
    }

    fn generated_modules() -> Vec<String> {
        let names = std::iter::once("Fixtures".to_owned())
            .chain((0..16).map(|index| format!("Shard{index:02}")))
            .collect::<Vec<_>>();
        ["Data", "Check"]
            .iter()
            .flat_map(|side| {
                names
                    .iter()
                    .map(move |name| format!("Semantics.Cards.{side}.{name}"))
            })
            .collect()
    }

    /// The shard whose data module `genCanon` wrote the card file into.
    fn shard_of(generated: &Path, file: &str) -> anyhow::Result<String> {
        let marker = format!("canon_card \"{file}\"");
        for index in 0..16 {
            let shard = format!("Shard{index:02}");
            let text = std::fs::read_to_string(generated.join(format!("Data/{shard}.lean")))?;
            if text.contains(&marker) {
                return Ok(shard);
            }
        }
        anyhow::bail!("genCanon wrote {file} into no shard")
    }

    fn copy_dir(from: &Path, to: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from).with_context(|| format!("reading {}", from.display()))? {
            let entry = entry?;
            let target = to.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_dir(&entry.path(), &target)?;
            } else {
                std::fs::copy(entry.path(), &target)?;
            }
        }
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
             SemanticsCardsData SemanticsCards SemanticsProofs'",
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

    #[test]
    fn the_regression_reads_built_modules_from_the_lake_log() {
        let log = "✔ [3/12] Built Semantics.Cards.Data.Shard03 (221ms)\n\
                   ✔ [4/12] Built Semantics.Cards.Check.Shard03 (2.4s)\n\
                   Build completed successfully (12 jobs).\n";
        assert_eq!(
            generation::built_modules(log),
            BTreeSet::from([
                "Semantics.Cards.Data.Shard03".to_owned(),
                "Semantics.Cards.Check.Shard03".to_owned(),
            ])
        );
    }
}
