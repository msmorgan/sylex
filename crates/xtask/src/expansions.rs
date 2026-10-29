//! Typed expansion snapshots of loose `plugins_v3` declarations.
//!
//! By default one invocation saves both builtin and canon, expanding canon over
//! the builtin prelude. Before a cleanup, save:
//! `cargo xtask expansions --out docs/evidence/plugins-v3-expansions/<ticket>/before`.
//! Make the change, then compare:
//! `cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/<ticket>/before`.
//! Comparison alone writes nothing; `--out <DIR>` also saves the new snapshot.
//! Existing snapshots require `--force` to overwrite, including legacy `_index.txt`
//! output. Unrelated nonempty directories are always refused. Repeatable
//! `--plugin <PATH>` selects specific bundles; `--no-prelude` loads them alone.
//!
//! Version 2 `snapshot.json` contains sorted records keyed by separate
//! `(bare_name, return_type, sample)` fields, independent of source paths. The
//! sample is the invocation argument string (empty for a closed or default-only
//! invocation). Every record includes whitespace-normalized `spelling` and
//! `lexical` fields, preserving absence. Names never become filesystem paths.
//!
//! Finite constructors, both booleans, absent/present options, empty/singleton
//! lists, default omission, and representative populated records and mixed
//! function results are sampled. Constructor fields use deterministic base
//! values. Each declaration has a cap of 256 invocations: individual parameter
//! variants precede lexicographic product combinations. The summary reports
//! capped combinations and omitted recursive constructors; this is a bounded
//! comparison. Registry snapshots include conferrals. Every sample must produce
//! a typed value; expansion failures make the command fail.
//!
//! Changed declarations print unified diffs including sample arguments, spelling,
//! lexical fields, and expanded values. Repeatable `--expect 'name : Type'` (or
//! an unambiguous bare name) accepts added, removed, or changed declarations.
//! Inspect each diff even when expected: allowances do not distinguish a spelling
//! edit from a semantic edit. Unused or duplicate allowances fail.
//!
//! `--rename 'old : Type=new : Type'` accepts declaration key renames. For names
//! containing `=`, use `--rename-file docs/evidence/plugins-v3-expansions/<ticket>/renames.json`,
//! a JSON array of `[old_key, new_key]` pairs. Maps are simultaneous, allowing
//! swaps and chains without collisions with unrenamed survivors. Accepted renames
//! have a separate count (`0 differences, 908 renames` succeeds). A renamed
//! declaration whose record also changes prints `changed <new_key>` and requires
//! `--expect <new_key>` too. Declaration renames normalize `<decl>.<param>` and
//! `<decl>.actor` mention IDs throughout callers. Parameter renames on declarations
//! with repeated `NounPhrase` parameters still change `<decl>.<param>` IDs in
//! callers and remain visible in their diffs.
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Args;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::bundle::BundleKey;
use deckmaste_semantics_v3::lower::Elaborator;
use deckmaste_semantics_v3::reader;
use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;

const INDEX_FILE: &str = "snapshot.json";
const LEGACY_INDEX_FILE: &str = "_index.txt";
const FORMAT_VERSION: u32 = 2;
type Outcomes = BTreeMap<BundleKey, Vec<Record>>;

#[derive(Debug, Args)]
pub struct ExpansionsArgs {
    /// Bundles to print (default: both builtin and canon), over the builtin prelude.
    #[arg(long)]
    plugin: Vec<PathBuf>,
    /// Output directory (default: target/expansions, or no writes with --compare).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Earlier keyed output directory to compare against, without modifying it.
    #[arg(long)]
    compare: Option<PathBuf>,
    /// Expected added, removed, or changed declaration: `name : Type` or bare `name`.
    #[arg(long, requires = "compare")]
    expect: Vec<String>,
    /// Key rename `OLD_KEY=NEW_KEY`; expanded values and arguments must agree.
    #[arg(long, requires = "compare")]
    rename: Vec<String>,
    /// JSON array of `[old_key, new_key]` pairs, also supporting names with `=`.
    #[arg(long, requires = "compare")]
    rename_file: Vec<PathBuf>,
    /// Load the bundles by themselves, for example a saved copy of builtin.
    #[arg(long)]
    no_prelude: bool,
    /// Replace an existing snapshot directory, including legacy `_index.txt` output.
    #[arg(long)]
    force: bool,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    bare_name: String,
    return_type: String,
    sample: String,
    spelling: Option<String>,
    lexical: Option<String>,
    outcome: Outcome,
}

struct Expansion {
    outcomes: Outcomes,
    limitations: BTreeSet<String>,
}

/// Produce a typed record for each declaration sample, retaining every failure.
#[must_use]
pub fn expand_all_keyed(bundle: &Bundle) -> BTreeMap<BundleKey, Vec<Record>> {
    expand_keys(bundle, bundle.items.keys()).outcomes
}

fn expand_keys<'a>(bundle: &Bundle, keys: impl Iterator<Item = &'a BundleKey>) -> Expansion {
    let reader = Elaborator::new(bundle);
    let expanded: Vec<_> = keys
        .collect::<Vec<_>>()
        .par_iter()
        .map(|key| {
            let declaration = &bundle.items[*key].declaration;
            let record = |sample: String, result: Result<String, String>| Record {
                bare_name: key.bare_name.to_string(),
                return_type: key.return_type.to_string(),
                sample,
                spelling: declaration
                    .fields
                    .get("spelling")
                    .map(|ts| reader::render(ts)),
                lexical: declaration
                    .fields
                    .get("lexical")
                    .map(|ts| reader::render(ts)),
                outcome: match result {
                    Ok(value) => Outcome::Printed(value),
                    Err(error) => Outcome::Failed(error),
                },
            };
            let (mut records, limitations) = match reader.sample_invocations(key) {
                Ok(samples) => {
                    let records = samples
                        .arguments
                        .into_iter()
                        .map(|arguments| {
                            let result = reader
                                .snapshot(key, &arguments, samples.result_type)
                                .map_err(|e| e.to_string());
                            record(arguments, result)
                        })
                        .collect::<Vec<_>>();
                    let limitations = samples
                        .limitations
                        .into_iter()
                        .map(|note| {
                            if note.starts_with("sample cap") {
                                format!("{}: {note}", key_label(key))
                            } else {
                                note
                            }
                        })
                        .collect::<Vec<_>>();
                    (records, limitations)
                }
                Err(error) => (
                    vec![record(String::new(), Err(error.to_string()))],
                    Vec::new(),
                ),
            };
            records.sort_by(|a, b| a.sample.cmp(&b.sample));
            ((*key).clone(), records, limitations)
        })
        .collect();
    let mut outcomes = BTreeMap::new();
    let mut limitations = BTreeSet::new();
    for (key, records, notes) in expanded {
        outcomes.insert(key, records);
        limitations.extend(notes);
    }
    Expansion {
        outcomes,
        limitations,
    }
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
        declarations: outcomes.values().flatten().cloned().collect(),
    };
    let mut bytes = serde_json::to_vec_pretty(&snapshot)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn read_snapshot(directory: &Path) -> anyhow::Result<Outcomes> {
    let file = directory.join(INDEX_FILE);
    let bytes = std::fs::read(&file)
        .with_context(|| format!("reading keyed snapshot {}", file.display()))?;
    let snapshot: Snapshot = serde_json::from_slice(&bytes)
        .with_context(|| format!("parsing keyed snapshot {}", file.display()))?;
    anyhow::ensure!(
        snapshot.version == FORMAT_VERSION,
        "unsupported snapshot version {} in {}",
        snapshot.version,
        file.display()
    );
    let mut outcomes = Outcomes::new();
    let mut seen = BTreeSet::new();
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
            seen.insert((key.clone(), record.sample.clone())),
            "duplicate baseline key: {} sample {:?}",
            key_label(&key),
            record.sample
        );
        outcomes.entry(key).or_default().push(record);
    }
    for records in outcomes.values_mut() {
        records.sort_by(|a, b| a.sample.cmp(&b.sample));
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

// Debug's BindingId is an Ident tuple. Rewrite only the string in an `id`
// field, never a spelling, label, invocation argument, or arbitrary AST string.
// Match corresponding mention occurrences against simultaneous rename pairs;
// overloaded declarations may share a source or target bare name.
fn rename_mentions(text: &str, after: &str, names: &[(String, String)]) -> String {
    let mut after_in_id = false;
    let mut new_ids = after.lines().filter(|line| {
        let is_id = after_in_id;
        after_in_id = line.trim() == "id: Ident(";
        is_id
    });
    let mut output = String::new();
    let mut in_id = false;
    for line in text.lines() {
        let mut rewritten = None;
        if in_id && let Some(new_id) = new_ids.next() {
            for (old, new) in names {
                let old = format!("{old}.");
                let new = format!("{new}.");
                let old = format!("{old:?}");
                let new = format!("{new:?}");
                if let Some(suffix) = line.trim_start().strip_prefix(old.trim_end_matches('"'))
                    && new_id.trim_start().strip_prefix(new.trim_end_matches('"')) == Some(suffix)
                {
                    rewritten = Some(format!(
                        "{}{}",
                        &line[..line.len() - line.trim_start().len()],
                        new_id.trim_start()
                    ));
                    break;
                }
            }
        }
        writeln!(output, "{}", rewritten.as_deref().unwrap_or(line)).unwrap();
        in_id = line.trim() == "id: Ident(";
    }
    output
}

fn record_text(records: &[Record]) -> String {
    let mut text = String::new();
    for record in records {
        writeln!(text, "// sample: {}", record.sample).unwrap();
        writeln!(
            text,
            "// spelling: {}",
            record.spelling.as_deref().unwrap_or("<absent>")
        )
        .unwrap();
        writeln!(
            text,
            "// lexical: {}",
            record.lexical.as_deref().unwrap_or("<absent>")
        )
        .unwrap();
        match &record.outcome {
            Outcome::Printed(value) => {
                text.push_str(value);
                if !value.ends_with('\n') {
                    text.push('\n');
                }
            }
            Outcome::Failed(error) => {
                writeln!(text, "FAILED {error}").unwrap();
            }
        }
    }
    text
}

// Find nearby shared lines without a quadratic allocation for large sampled
// declarations. The bounded lookahead affects alignment, never retained content.
fn unified_changes<'a>(before: &[&'a str], after: &[&'a str]) -> Vec<(char, &'a str)> {
    let (mut old, mut new) = (0, 0);
    let mut changes = Vec::new();
    while old < before.len() && new < after.len() {
        if before[old] == after[new] {
            changes.push((' ', before[old]));
            old += 1;
            new += 1;
            continue;
        }
        let anchor = (1usize..=64).find_map(|distance| {
            (distance.saturating_sub(32)..=distance.min(32)).find_map(|left| {
                let right = distance - left;
                match (before.get(old + left), after.get(new + right)) {
                    (Some(a), Some(b)) if a == b => Some((left, right)),
                    _ => None,
                }
            })
        });
        let (left, right) = anchor.unwrap_or((1, 1));
        changes.extend(before[old..old + left].iter().map(|line| ('-', *line)));
        changes.extend(after[new..new + right].iter().map(|line| ('+', *line)));
        old += left;
        new += right;
    }
    changes.extend(before[old..].iter().map(|line| ('-', *line)));
    changes.extend(after[new..].iter().map(|line| ('+', *line)));
    changes
}

fn unified_diff(old: &str, new: &str, old_label: &str, new_label: &str) -> String {
    let before: Vec<_> = old.lines().collect();
    let after: Vec<_> = new.lines().collect();
    let changes = unified_changes(&before, &after);
    let mut hunks: Vec<(usize, usize)> = Vec::new();
    for (index, _) in changes
        .iter()
        .enumerate()
        .filter(|(_, (kind, _))| *kind != ' ')
    {
        let start = index.saturating_sub(3);
        let end = (index + 4).min(changes.len());
        if let Some((_, previous_end)) = hunks.last_mut()
            && start <= *previous_end
        {
            *previous_end = end;
        } else {
            hunks.push((start, end));
        }
    }
    if hunks.is_empty() {
        return String::new();
    }
    let mut diff = format!("--- {old_label}\n+++ {new_label}\n");
    let (mut old_line, mut new_line, mut previous) = (0, 0, 0);
    for (start, end) in hunks {
        for (kind, _) in &changes[previous..start] {
            old_line += usize::from(*kind != '+');
            new_line += usize::from(*kind != '-');
        }
        let old_count = changes[start..end]
            .iter()
            .filter(|(kind, _)| *kind != '+')
            .count();
        let new_count = changes[start..end]
            .iter()
            .filter(|(kind, _)| *kind != '-')
            .count();
        writeln!(
            diff,
            "@@ -{},{} +{},{} @@",
            old_line + usize::from(old_count > 0),
            old_count,
            new_line + usize::from(new_count > 0),
            new_count
        )
        .unwrap();
        for (kind, line) in &changes[start..end] {
            writeln!(diff, "{kind}{line}").unwrap();
        }
        old_line += old_count;
        new_line += new_count;
        previous = end;
    }
    diff
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
    renames: BTreeMap<BundleKey, BundleKey>,
    diffs: BTreeMap<BundleKey, String>,
    errors: Vec<String>,
}

impl Comparison {
    fn ensure_success(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.errors.is_empty(), "{}", self.errors.join("\n"));
        Ok(())
    }

    fn report(&self) -> String {
        let mut report = String::new();
        for (old, new) in &self.renames {
            writeln!(report, "renamed {} -> {}", key_label(old), key_label(new)).unwrap();
        }
        for difference in &self.differences {
            writeln!(
                report,
                "{} {}",
                difference.label(),
                key_label(difference.key())
            )
            .unwrap();
            if let Some(diff) = self.diffs.get(difference.key()) {
                report.push_str(diff);
            }
        }
        writeln!(
            report,
            "{} differences, {} renames",
            self.differences.len(),
            self.renames.len()
        )
        .unwrap();
        report
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
    let mut mapping = BTreeMap::new();
    let mut targets = BTreeSet::new();
    let mut names = Vec::new();
    for (old, new) in renames {
        let old = resolve_key(old, &old_keys)?;
        let new = resolve_key(new, &new_keys)?;
        anyhow::ensure!(old != new, "rename endpoints must differ");
        anyhow::ensure!(
            !mapping.contains_key(&old) && targets.insert(new.clone()),
            "duplicate rename endpoint"
        );
        names.push((old.bare_name.to_string(), new.bare_name.to_string()));
        mapping.insert(old, new);
    }
    for (old, new) in &mapping {
        anyhow::ensure!(
            (!new_keys.contains(old) || targets.contains(old))
                && (!old_keys.contains(new) || mapping.contains_key(new)),
            "rename {} -> {} collides with an unrenamed surviving key",
            key_label(old),
            key_label(new)
        );
    }
    let mut differences = Vec::new();
    let mut diffs = BTreeMap::new();
    let mut paired = BTreeSet::new();
    for (old, records) in before {
        let new = mapping.get(old).unwrap_or(old);
        match after.get(new) {
            Some(current) => {
                paired.insert(new.clone());
                let new_text = record_text(current);
                let old_text = rename_mentions(&record_text(records), &new_text, &names);
                if old_text != new_text {
                    differences.push(Difference::Changed(new.clone()));
                    diffs.insert(
                        new.clone(),
                        unified_diff(
                            &old_text,
                            &new_text,
                            &format!("before/{}", key_label(old)),
                            &format!("after/{}", key_label(new)),
                        ),
                    );
                }
            }
            None => differences.push(Difference::Removed(old.clone())),
        }
    }
    for new in after.keys().filter(|key| !paired.contains(*key)) {
        differences.push(Difference::Added(new.clone()));
    }
    differences.sort_by(|a, b| a.key().cmp(b.key()).then_with(|| a.label().cmp(b.label())));
    let mut errors = Vec::new();
    let mut accepted = BTreeSet::new();
    for selector in expected {
        let key = match resolve_key(selector, &keys) {
            Ok(key) => key,
            Err(error) => {
                errors.push(format!("unused expectation {selector:?}: {error}"));
                continue;
            }
        };
        if !differences.iter().any(|diff| diff.key() == &key) || !accepted.insert(key) {
            errors.push(format!("unused expectation {selector:?}"));
        }
    }
    for diff in &differences {
        if !accepted.contains(diff.key()) {
            errors.push(format!(
                "unexpected {} {}",
                diff.label(),
                key_label(diff.key())
            ));
        }
    }
    Ok(Comparison {
        differences,
        renames: mapping,
        diffs,
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
/// Reports loading, expansion, comparison, or output errors. Existing snapshots
/// require `--force`; unrelated directories and baseline overlaps are refused.
pub fn run(args: &ExpansionsArgs) -> anyhow::Result<()> {
    run_with_default_output(args, &default_out_dir())
}

fn run_with_default_output(args: &ExpansionsArgs, default_output: &Path) -> anyhow::Result<()> {
    let baseline = args.compare.as_deref().map(read_snapshot).transpose()?;
    let output = args
        .out
        .clone()
        .or_else(|| args.compare.is_none().then(|| default_output.to_owned()));
    if let (Some(before), Some(out)) = (&args.compare, &output) {
        let before = resolved_path(before)?;
        let out = resolved_path(out)?;
        anyhow::ensure!(
            !out.starts_with(&before) && !before.starts_with(&out),
            "output and baseline directories must not overlap"
        );
    }
    let (bundle, keys) = load_plugins(args)?;
    let expansion = expand_keys(&bundle, keys.iter());
    let outcomes = expansion.outcomes;
    let failures: Vec<_> = outcomes
        .iter()
        .flat_map(|(key, records)| {
            records.iter().filter_map(move |record| {
                if let Outcome::Failed(error) = &record.outcome {
                    Some(format!(
                        "{} sample {:?}: {error}",
                        key_label(key),
                        record.sample
                    ))
                } else {
                    None
                }
            })
        })
        .collect();
    let samples: usize = outcomes.values().map(Vec::len).sum();
    println!(
        "{} declarations, {samples} samples: {} printed, {} failed; sample cap 256 per declaration",
        outcomes.len(),
        samples - failures.len(),
        failures.len()
    );
    for note in expansion.limitations {
        println!("sampling: {note}");
    }
    for failure in &failures {
        eprintln!("FAILED {failure}");
    }
    if let Some(out) = output {
        let bytes = snapshot_bytes(&outcomes)?;
        prepare_out_dir(&out, args.force)?;
        std::fs::write(out.join(INDEX_FILE), bytes)?;
        println!("written to {}", out.display());
    }
    anyhow::ensure!(
        failures.is_empty(),
        "{} samples failed to expand",
        failures.len()
    );
    if let Some(before) = baseline {
        let comparison = compare(&before, &outcomes, &args.expect, &read_renames(args)?)?;
        print!("{}", comparison.report());
        comparison.ensure_success()?;
    }
    Ok(())
}

fn load_plugins(args: &ExpansionsArgs) -> anyhow::Result<(Bundle, BTreeSet<BundleKey>)> {
    let builtin = default_plugin_dir().canonicalize()?;
    let roots = if args.plugin.is_empty() {
        vec![builtin.clone(), workspace_root().join("plugins_v3/canon")]
    } else {
        args.plugin.clone()
    };
    let mut items = Vec::new();
    let mut keys = BTreeSet::new();
    let mut loaded_builtin = false;
    for root in roots {
        let root = root
            .canonicalize()
            .with_context(|| format!("resolving {}", root.display()))?;
        loaded_builtin |= root == builtin;
        let bundle = Bundle::load(&root)?;
        keys.extend(bundle.items.keys().cloned());
        items.extend(bundle.items.into_values());
    }
    if !args.no_prelude && !loaded_builtin {
        items.extend(Bundle::load(&builtin)?.items.into_values());
    }
    Ok((Bundle::from_items(items)?, keys))
}

fn read_renames(args: &ExpansionsArgs) -> anyhow::Result<Vec<(String, String)>> {
    let mut renames = Vec::new();
    for rename in &args.rename {
        let (old, new) = rename
            .split_once('=')
            .context("rename requires OLD_KEY=NEW_KEY")?;
        renames.push((old.to_owned(), new.to_owned()));
    }
    for file in &args.rename_file {
        let bytes = std::fs::read(file)
            .with_context(|| format!("reading rename file {}", file.display()))?;
        renames.extend(
            serde_json::from_slice::<Vec<(String, String)>>(&bytes)
                .with_context(|| format!("parsing rename file {}", file.display()))?,
        );
    }
    Ok(renames)
}

fn prepare_out_dir(out: &Path, force: bool) -> anyhow::Result<()> {
    if out.exists() {
        let empty = out.read_dir()?.next().is_none();
        let snapshot = out.join(INDEX_FILE).is_file() || out.join(LEGACY_INDEX_FILE).is_file();
        anyhow::ensure!(
            empty || snapshot,
            "{} is not empty and holds no `{INDEX_FILE}` or `{LEGACY_INDEX_FILE}`; refusing to replace it",
            out.display()
        );
        anyhow::ensure!(
            !snapshot || force,
            "{} already contains a snapshot; use --force to overwrite it",
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
        let outcomes = expand_all_keyed(&bundle);
        assert_eq!(outcomes.len(), bundle.items.len());
        let failed: Vec<_> = outcomes
            .iter()
            .filter(|(_, records)| {
                records
                    .iter()
                    .any(|record| matches!(record.outcome, Outcome::Failed(_)))
            })
            .collect();
        assert!(failed.is_empty(), "{failed:#?}");
    }

    #[test]
    fn a_helper_spelling_and_its_constructor_spelling_print_the_same() {
        let bundle = Bundle::load(&default_plugin_dir()).unwrap();
        let reader = Elaborator::new(&bundle);
        // Constructor basis checked against Lean's reduction of amass Zombie 3.
        let basis = r#"
            have creature : CardType := { name := "Creature", cardClass := some .permanentCard, stats := [.power, .toughness] };
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
    fn output_directory_requires_an_expansion_snapshot() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output");
        std::fs::create_dir(&output).unwrap();
        std::fs::write(output.join("keep"), "original").unwrap();
        assert!(prepare_out_dir(&output, false).is_err());
        assert!(prepare_out_dir(&output, true).is_err());
        assert_eq!(
            std::fs::read_to_string(output.join("keep")).unwrap(),
            "original"
        );
        std::fs::write(output.join(INDEX_FILE), "").unwrap();
        assert!(prepare_out_dir(&output, false).is_err());
        prepare_out_dir(&output, true).unwrap();
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
            expand_all_keyed(&bundle)
                .values()
                .flatten()
                .next()
                .unwrap()
                .outcome,
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
            plugin: vec![plugin.to_owned()],
            out: out.map(Path::to_owned),
            compare: baseline.map(Path::to_owned),
            expect: Vec::new(),
            rename: Vec::new(),
            rename_file: Vec::new(),
            no_prelude: true,
            force: false,
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
        let default_output = temp.path().join("target/expansions");
        std::fs::create_dir(plugin.join("nested")).unwrap();
        std::fs::rename(plugin.join("before.lean"), plugin.join("nested/after.lean")).unwrap();
        let after = expand_all_keyed(&Bundle::load(&plugin).unwrap());
        let comparison = compare(&read_snapshot(&baseline).unwrap(), &after, &[], &[]).unwrap();
        assert_eq!(comparison.differences, []);
        comparison.ensure_success().unwrap();
        run_with_default_output(&args(&plugin, None, Some(&baseline)), &default_output).unwrap();
        assert_eq!(std::fs::read(baseline.join(INDEX_FILE)).unwrap(), original);
        assert!(!default_output.exists());
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
                .flatten()
                .all(|record| matches!(record.outcome, Outcome::Printed(_)))
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
        let mut record = before.values().next().unwrap()[0].clone();
        record.return_type = "Bool".into();
        record.outcome = Outcome::Printed("false".into());
        overloaded.insert(key, vec![record]);
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
        assert!(run(&comparison).is_err());
        comparison.force = true;
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
    #[test]
    fn spelling_and_lexical_edits_have_readable_unified_diffs() {
        let make = |fields: &str| {
            Bundle::from_items(vec![BundleItem {
                declaration: reader::parse(&format!(
                    "primitive value : Nat := {{ body := 3 {fields} }}"
                ))
                .unwrap(),
                file: "example.lean".into(),
            }])
            .unwrap()
        };
        let before = expand_all_keyed(&make(
            ", spelling := \"first\", lexical := { form := \"noun\" }",
        ));
        let whitespace = expand_all_keyed(&make(
            ",\n spelling := \"first\", lexical := {form:=\"noun\"}",
        ));
        compare(&before, &whitespace, &[], &[])
            .unwrap()
            .ensure_success()
            .unwrap();
        for fields in [
            ", spelling := \"second\", lexical := { form := \"noun\" }",
            ", lexical := { form := \"noun\" }",
            ", spelling := \"first\"",
            ", spelling := \"first\", lexical := { form := \"verb\" }",
        ] {
            let after = expand_all_keyed(&make(fields));
            for (old, new) in [(&before, &after), (&after, &before)] {
                let comparison = compare(old, new, &[], &[]).unwrap();
                assert!(comparison.ensure_success().is_err());
                assert_eq!(comparison.differences.len(), 1);
                let report = comparison.report();
                assert!(
                    report.contains(
                        "changed value : Nat\n--- before/value : Nat\n+++ after/value : Nat\n@@ "
                    ),
                    "{report}"
                );
                assert!(
                    report
                        .lines()
                        .any(|l| l.starts_with("-// spelling:") || l.starts_with("-// lexical:")),
                    "{report}"
                );
                assert!(
                    report
                        .lines()
                        .any(|l| l.starts_with("+// spelling:") || l.starts_with("+// lexical:")),
                    "{report}"
                );
                compare(old, new, &["value".into()], &[])
                    .unwrap()
                    .ensure_success()
                    .unwrap();
            }
        }
    }

    #[test]
    fn sample_records_round_trip_and_detect_nondefault_branches() {
        let make = |yes| {
            Bundle::from_items(vec![BundleItem {
            declaration: reader::parse(&format!("template value (flag : Bool := false) (items : List Nat := []) (maybe : Option Nat := none) : Nat := {{ body := if flag then {yes} else 3 }}")).unwrap(),
            file: "example.lean".into(),
        }]).unwrap()
        };
        let before = expand_all_keyed(&make(4));
        let records = before.values().next().unwrap();
        assert_eq!(records.len(), 27);
        assert!(records.iter().any(|r| r.sample.is_empty()));
        assert!(records.iter().any(|r| r.sample.contains("flag := true")
            && r.sample.contains("items := [")
            && r.sample.contains("maybe := some")));
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join(INDEX_FILE),
            snapshot_bytes(&before).unwrap(),
        )
        .unwrap();
        assert_eq!(read_snapshot(temp.path()).unwrap(), before);
        let after = expand_all_keyed(&make(5));
        let comparison = compare(&before, &after, &[], &[]).unwrap();
        assert_eq!(comparison.differences.len(), 1);
        assert!(comparison.report().contains("-4\n+5\n"));
        assert!(comparison.ensure_success().is_err());
    }

    fn union(fixtures: &[(&str, &str)]) -> Outcomes {
        fixtures
            .iter()
            .flat_map(|(name, value)| expand_all_keyed(&fixture(name, value)))
            .collect()
    }

    #[test]
    fn simultaneous_renames_allow_swaps_and_chains_and_reject_survivor_collisions() {
        let before = union(&[("a", "3"), ("b", "4")]);
        for (after, renames) in [
            (
                union(&[("a", "4"), ("b", "3")]),
                vec![("a".into(), "b".into()), ("b".into(), "a".into())],
            ),
            (
                union(&[("b", "3"), ("c", "4")]),
                vec![("a".into(), "b".into()), ("b".into(), "c".into())],
            ),
        ] {
            let comparison = compare(&before, &after, &[], &renames).unwrap();
            comparison.ensure_success().unwrap();
            assert_eq!(comparison.differences, []);
            assert_eq!(comparison.renames.len(), 2);
            assert!(comparison.report().contains("0 differences, 2 renames"));
        }
        assert!(compare(&before, &before, &[], &[("a".into(), "b".into())]).is_err());
        let after = union(&[("a", "4"), ("c", "3")]);
        assert!(compare(&before, &after, &[], &[("a".into(), "c".into())]).is_err());
    }

    #[test]
    fn overloaded_declaration_renames_do_not_require_unique_bare_names() {
        let make = |first: &str, second: &str| {
            Bundle::from_items([
            format!("template {first} (who : NounPhrase) : Action := {{ body := [.instruct who [], .instruct who []] }}"),
            format!("template {second} (who : NounPhrase) : Ability := {{ body := .spell none [.instruct who [], .instruct who []] }}"),
            format!("template caller : Ability := {{ body := .spell none ({first} .you) }}"),
            format!("template otherCaller : Ability := {{ body := {second} .you }}"),
        ].iter().enumerate().map(|(i, source)| BundleItem { declaration: reader::parse(source).unwrap(), file: format!("{i}.lean").into() }).collect()).unwrap()
        };
        let before = expand_all_keyed(&make("old", "old"));
        let after = expand_all_keyed(&make("newAction", "newAbility"));
        let renames = [
            ("old : Action".into(), "newAction : Action".into()),
            ("old : Ability".into(), "newAbility : Ability".into()),
        ];
        let comparison = compare(&before, &after, &[], &renames).unwrap();
        comparison.ensure_success().unwrap();
        assert_eq!(comparison.renames.len(), 2);
        assert_eq!(comparison.differences, []);
        // The other overload may survive under its original bare name too.
        let after = expand_all_keyed(&make("newAction", "old"));
        compare(&before, &after, &[], &renames[..1])
            .unwrap()
            .ensure_success()
            .unwrap();
    }

    #[test]
    fn renamed_edits_report_one_rename_and_a_changed_diff() {
        let before = expand_all_keyed(&fixture("old", "3"));
        let after = expand_all_keyed(&fixture("new", "4"));
        let renames = [("old".into(), "new".into())];
        let comparison = compare(&before, &after, &[], &renames).unwrap();
        assert_eq!(comparison.renames.len(), 1);
        assert_eq!(
            comparison.differences,
            [Difference::Changed(after.keys().next().unwrap().clone())]
        );
        let report = comparison.report();
        assert!(report.contains("renamed old : Nat -> new : Nat\nchanged new : Nat\n--- before/old : Nat\n+++ after/new : Nat\n@@ "), "{report}");
        assert!(report.contains("-3\n+4\n"));
        assert!(report.contains("1 differences, 1 renames"));
        assert!(comparison.ensure_success().is_err());
        compare(&before, &after, &["new".into()], &renames)
            .unwrap()
            .ensure_success()
            .unwrap();
    }

    #[test]
    fn declaration_renames_normalize_mentions_in_callers_but_preserve_parameter_changes() {
        let make = |name: &str, parameter: &str| {
            Bundle::from_items([
            format!("template {name} ({parameter} : NounPhrase) : Action := {{ body := [.instruct {parameter} [], .instruct {parameter} []] }}"),
            format!("action actor{name} := {{ deed := none, body := [.instruct theActor [], .instruct theActor []] }}"),
            format!("template caller : Action := {{ body := [{name} .you, actor{name}] }}"),
        ].iter().enumerate().map(|(i, source)| BundleItem { declaration: reader::parse(source).unwrap(), file: format!("{i}.lean").into() }).collect()).unwrap()
        };
        let before = expand_all_keyed(&make("old", "who"));
        let after = expand_all_keyed(&make("new", "who"));
        let renames = [
            ("old".into(), "new".into()),
            ("actorold".into(), "actornew".into()),
        ];
        let comparison = compare(&before, &after, &[], &renames).unwrap();
        comparison.ensure_success().unwrap();
        assert_eq!(comparison.differences, []);
        let changed = expand_all_keyed(&make("new", "subject"));
        let comparison = compare(&before, &changed, &[], &renames).unwrap();
        assert!(comparison.ensure_success().is_err());
        assert!(
            comparison
                .differences
                .iter()
                .any(|d| d.key().bare_name.as_str() == "caller")
        );
        let text = "name: Ident(\n    \"old.who\",\n),\nid: Ident(\n    \"old.who\",\n),\n";
        let names = [("old".into(), "new".into()), ("new".into(), "third".into())];
        let after = text.replace("id: Ident(\n    \"old.who\"", "id: Ident(\n    \"new.who\"");
        let normalized = rename_mentions(text, &after, &names);
        assert!(normalized.starts_with("name: Ident(\n    \"old.who\""));
        assert!(normalized.contains("id: Ident(\n    \"new.who\""));
        assert!(!normalized.contains("third.who"));
        let names = [
            ("old".into(), "new".into()),
            ("old.deep".into(), "nested".into()),
        ];
        assert!(
            rename_mentions(
                "id: Ident(\n    \"old.deep.who\",\n),\n",
                "id: Ident(\n    \"nested.who\",\n),\n",
                &names
            )
            .contains("nested.who")
        );
    }

    #[test]
    fn existing_and_legacy_snapshots_require_force() {
        let temp = tempfile::tempdir().unwrap();
        for marker in [INDEX_FILE, LEGACY_INDEX_FILE] {
            let output = temp.path().join(marker);
            prepare_out_dir(&output, false).unwrap();
            std::fs::write(output.join(marker), "baseline").unwrap();
            assert!(
                prepare_out_dir(&output, false)
                    .unwrap_err()
                    .to_string()
                    .contains("--force")
            );
            assert_eq!(
                std::fs::read_to_string(output.join(marker)).unwrap(),
                "baseline"
            );
            prepare_out_dir(&output, true).unwrap();
            assert!(output.read_dir().unwrap().next().is_none());
        }
    }

    fn copy_tree(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).unwrap();
        for entry in from.read_dir().unwrap() {
            let entry = entry.unwrap();
            let target = to.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }

    #[test]
    fn real_builtin_dumps_are_identical_and_file_moves_preserve_expansions() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("builtin");
        copy_tree(&default_plugin_dir(), &plugin);
        let first = temp.path().join("first");
        let second = temp.path().join("second");
        run(&args(&plugin, Some(&first), None)).unwrap();
        run(&args(&plugin, Some(&second), None)).unwrap();
        assert_eq!(
            std::fs::read(first.join(INDEX_FILE)).unwrap(),
            std::fs::read(second.join(INDEX_FILE)).unwrap()
        );
        std::fs::rename(
            plugin.join("templates/ability/ptDefinition.lean"),
            plugin.join("pt-renamed.lean"),
        )
        .unwrap();
        run_with_default_output(
            &args(&plugin, None, Some(&first)),
            &temp.path().join("unused"),
        )
        .unwrap();
        assert!(!temp.path().join("unused").exists());
    }

    #[test]
    fn toughness_edits_and_bogus_nonfirst_list_elements_are_detected() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("builtin");
        copy_tree(&default_plugin_dir(), &plugin);
        let file = plugin.join("templates/ability/ptDefinition.lean");
        let original = std::fs::read_to_string(&file).unwrap();
        let bundle = Bundle::load(&plugin).unwrap();
        let key = bundle
            .items
            .keys()
            .find(|k| k.bare_name.as_str() == "ptDefinition")
            .unwrap()
            .clone();
        let before = expand_keys(&bundle, std::iter::once(&key)).outcomes;
        let marker = "stat toughness (set amount)";
        assert_eq!(original.matches(marker).count(), 1);
        std::fs::write(&file, original.replace(marker, "stat toughness (up amount)")).unwrap();
        let edited = expand_keys(&Bundle::load(&plugin).unwrap(), std::iter::once(&key)).outcomes;
        let comparison = compare(&before, &edited, &[], &[]).unwrap();
        assert_eq!(comparison.differences, [Difference::Changed(key.clone())]);
        assert!(comparison.report().contains("Up"));
        assert!(comparison.ensure_success().is_err());
        std::fs::write(&file, original.replace(marker, "totallyBogusIdentifier 1 2 3")).unwrap();
        let broken = expand_keys(&Bundle::load(&plugin).unwrap(), std::iter::once(&key)).outcomes;
        assert!(broken[&key].iter().any(|r| matches!(&r.outcome, Outcome::Failed(error) if error.contains("totallyBogusIdentifier"))));
        assert!(
            compare(&before, &broken, &[], &[])
                .unwrap()
                .ensure_success()
                .is_err()
        );
    }

    #[test]
    fn default_dump_contains_builtin_and_canon_with_disjoint_keys() {
        let temp = tempfile::tempdir().unwrap();
        let mut options = args(temp.path(), Some(temp.path()), None);
        options.plugin.clear();
        options.no_prelude = false;
        run(&options).unwrap();
        let snapshot = read_snapshot(temp.path()).unwrap();
        let builtin = Bundle::load(&default_plugin_dir()).unwrap();
        let canon = Bundle::load(&workspace_root().join("plugins_v3/canon")).unwrap();
        assert_eq!(snapshot.len(), builtin.items.len() + canon.items.len());
        assert!(builtin.items.keys().all(|key| snapshot.contains_key(key)));
        assert!(canon.items.keys().all(|key| snapshot.contains_key(key)));
        assert_eq!(canon.items.len(), 908);
        options.out = None;
        options.compare = Some(temp.path().to_owned());
        run(&options).unwrap();
    }

    #[test]
    fn explicit_canon_loads_over_builtin_and_repeated_plugin_paths_work() {
        let temp = tempfile::tempdir().unwrap();
        let canon = workspace_root().join("plugins_v3/canon");
        let mut options = args(&canon, Some(temp.path()), None);
        options.no_prelude = false;
        run(&options).unwrap();
        let snapshot = read_snapshot(temp.path()).unwrap();
        assert_eq!(snapshot.len(), 908);
        assert!(
            snapshot
                .keys()
                .all(|key| key.return_type.as_str() == "Spelled")
        );
        let first = temp.path().join("one");
        let second = temp.path().join("two");
        std::fs::create_dir(&first).unwrap();
        std::fs::create_dir(&second).unwrap();
        std::fs::write(first.join("a.lean"), "primitive a : Nat := { body := 3 }").unwrap();
        std::fs::write(second.join("b.lean"), "primitive b : Nat := { body := 4 }").unwrap();
        let mut options = args(&first, None, None);
        options.plugin.push(second);
        let (bundle, keys) = load_plugins(&options).unwrap();
        assert_eq!(keys.len(), 2);
        assert_eq!(bundle.items.len(), 2);
    }

    #[test]
    fn nine_hundred_eight_simultaneous_renames_report_zero_differences_and_succeed() {
        let temp = tempfile::tempdir().unwrap();
        let plugin = temp.path().join("plugin");
        std::fs::create_dir(&plugin).unwrap();
        let baseline = temp.path().join("baseline");
        let renames = (0..908)
            .map(|i| (format!("value{i}"), format!("value{}", (i + 1) % 908)))
            .collect::<Vec<_>>();
        for i in 0..908 {
            std::fs::write(
                plugin.join(format!("{i}.lean")),
                format!("primitive value{i} : Nat := {{ body := {i} }}"),
            )
            .unwrap();
        }
        run(&args(&plugin, Some(&baseline), None)).unwrap();
        for (i, (_, new)) in renames.iter().enumerate() {
            std::fs::write(
                plugin.join(format!("{i}.lean")),
                format!("primitive {new} : Nat := {{ body := {i} }}"),
            )
            .unwrap();
        }
        let before = read_snapshot(&baseline).unwrap();
        let after = expand_all_keyed(&Bundle::load(&plugin).unwrap());
        let comparison = compare(&before, &after, &[], &renames).unwrap();
        assert_eq!(comparison.differences, []);
        assert_eq!(comparison.renames.len(), 908);
        assert!(
            comparison
                .report()
                .ends_with("0 differences, 908 renames\n")
        );
        comparison.ensure_success().unwrap();
        let file = temp.path().join("renames.json");
        std::fs::write(&file, serde_json::to_vec(&renames).unwrap()).unwrap();
        let mut options = args(&plugin, None, Some(&baseline));
        options.rename_file.push(file);
        run(&options).unwrap();
    }

    #[test]
    fn compare_and_rename_file_errors_include_the_path() {
        let temp = tempfile::tempdir().unwrap();
        let error = read_snapshot(temp.path()).unwrap_err().to_string();
        assert!(error.contains(&temp.path().join(INDEX_FILE).display().to_string()));
        let file = temp.path().join("missing.json");
        let mut options = args(temp.path(), None, None);
        options.rename_file.push(file.clone());
        assert!(
            read_renames(&options)
                .unwrap_err()
                .to_string()
                .contains(&file.display().to_string())
        );
        std::fs::write(&file, "malformed").unwrap();
        assert!(
            read_renames(&options)
                .unwrap_err()
                .to_string()
                .contains(&file.display().to_string())
        );
    }
    #[test]
    fn metadata_only_diffs_keep_sampled_expansions_as_context() {
        let make = |spelling: &str| {
            Bundle::from_items(vec![BundleItem {
            declaration: reader::parse(&format!("primitive value (flag : Bool) : Nat := {{ body := 3, spelling := \"{spelling}\" }}")).unwrap(),
            file: "example.lean".into(),
        }]).unwrap()
        };
        let before = expand_all_keyed(&make("first"));
        let after = expand_all_keyed(&make("second"));
        let comparison = compare(&before, &after, &[], &[]).unwrap();
        let report = comparison.report();
        let edits = report
            .lines()
            .filter(|line| {
                (line.starts_with('-') || line.starts_with('+'))
                    && !line.starts_with("---")
                    && !line.starts_with("+++")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            edits,
            [
                "-// spelling: \"first\"",
                "+// spelling: \"second\"",
                "-// spelling: \"first\"",
                "+// spelling: \"second\""
            ]
        );
        assert!(report.contains(" 3\n"));
    }
}
