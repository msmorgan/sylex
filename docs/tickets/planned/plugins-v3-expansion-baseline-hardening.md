---
needs: [plugins-v3-expansion-baseline]
---
# Make the expansion baseline a gate the `plugins-v3-*` cleanups can trust

Standard constraints apply. A review of `plugins-v3-expansion-baseline` (`wpnzmoxz`, `pxkoxyuq`)
found three things that work:

- Two dumps of an unchanged tree are byte-identical.
- Moves and renames pass cleanly.
- A bulk `Semantics.` unqualification flagged only its two real mis-resolutions, `hideaway` and
  `lookAt`.

The same review found that the builtin dump misses changes in meaning, and that the workflow is
documented nowhere a cleanup reads it. Every cleanup gates on this tool, so fix it before any of
them runs. Line references below are to `crates/xtask/src/expansions.rs` at `pxkoxyuq` unless
stated.

## Defects

1. **Changes in meaning go undetected in branches the dump never evaluates (high).**
   - `lower/snapshot.rs` (`:135-174`) expands each builtin declaration once, with one sample
     per parameter: `Bool` is `false`, `Option` is `none`, `List` is `[]`, and an enum is its
     first constructor with the fewest fields.
   - A change in any other `match` arm or `if` branch therefore passes. Changing
     `ptDefinition`'s `bothEach` arm from `sets` to `adds` gave 0 differences. So did replacing
     another arm with `totallyBogusIdentifier 1 2 3`.
   - `plugins-v3-builtin-cleanup` step 2 rewrites data-type leaves inside exactly such arms.
   - Fix: expand every constructor of each finite enum parameter, `Bool` both ways, `Option`
     as `none` and as `some`, and `List` empty and with one element. Write one keyed record per
     sample, e.g. `(bare_name, return_type, sample)`.
   - Where the sample space must be capped, the cap and what it leaves out are printed in the
     summary, not silently skipped.
2. **`spelling` and `lexical` are not recorded (medium).** A cleanup that damages a `lexical`
   frame or a `spelling` string reports 0 differences, and those fields feed the English
   catalogs. Record them in each declaration's record, as written, after normalizing
   whitespace.
3. **Diffs can't be read (medium).** A changed key prints only `changed <key>`. Print a unified
   diff of the old and new expansion for each changed key, and for each renamed key whose
   expansion also changed. The old mode's `diff -r` workflow was removed and needs this
   replacement.
4. **Renames count as differences (medium).**
   - An accepted rename prints as `removed`/`added` and counts toward "N differences". The 908
     card renames in `plugins-v3-canon-cleanup` would read as "1816 differences" against a
     "zero changes" criterion.
   - Report accepted renames as `renamed old -> new` with their own count. Exit 0 when the only
     differences are accepted renames and accepted expectations.
   - Allow swaps and chains: drop the rule at `:252` that a rename target may not be an old key.
     Resolve the rename map as one simultaneous substitution instead.
5. **The baseline can be overwritten by accident (medium).** Re-running the save command after
   making the change silently replaces the baseline (`prepare_out_dir`, `:425`). Refuse to
   overwrite an existing snapshot unless `--force` is given.
6. **One baseline directory but two dumps (medium).** The builtin dump and the canon dump
   (`--plugin plugins_v3/canon`) are separate, and the second overwrites the first in the
   documented `before/`.
   - Either write both in one invocation (`--plugin` repeated, or an `--all` that dumps builtin
     and canon), or document `before/builtin` and `before/canon`.
   - Prefer the single invocation, so a cleanup cannot forget canon.
7. **The snapshot is named `_index.txt` (low).** That name is left over from the old mode's
   plain-text index; the file is now JSON. Rename it to `snapshot.json` and bump
   `FORMAT_VERSION`. `prepare_out_dir` keeps recognising `_index.txt` as an earlier output so
   old directories can be cleared.
8. **Leftovers and error messages (low).**
   - Delete `expand_all` (`:110`): only tests use it, and it collapses two declarations in one
     file.
   - Delete the hard-coded `"0 skipped"` (`:383`).
   - Add the path to errors from reading `--compare` (`:146`; only the parse has context) and
     from reading and parsing `--rename-file` (`:412`).
   - Put blank lines between the items at `:119-330` to match the rest of the file. Make
     `compare`'s handling of `accepted` (`:294`) mutate in one place.
9. **Mention ids embed declaration and parameter names (low, decide).**
   - `lower/mod.rs:1037` builds ids as `<decl>.<param>`. Renaming a definition, or a parameter
     of a template with a repeated noun-phrase parameter, therefore shows up as a change in
     every caller. None of the currently planned renames hit this.
   - Either apply `--rename` to these ids during comparison, or record the limitation in the
     workflow docs. If parameter renames are meant to be invisible, do the former.

## Tests

- **Canon-over-builtin path:** cover the branch of `run` that loads canon over the builtin
  prelude. Every current helper sets `no_prelude: true` (`:556`).
- **Real tree:** run a byte-identical dump and a pure file rename on the real
  `plugins_v3/builtin`, using a temporary copy, not just toy bundles.
- **The `bothEach` regression:** an edit in a non-first `match` arm of a copied builtin
  template is reported as changed.
- **Isolate from the workspace's `target/`.**
  `moving_a_lean_file_reports_no_changes_and_compare_writes_nothing` reads the real
  `target/expansions` (`:595,605`). Use a temporary directory instead.
- **Existing tests:** these assert what they should. Keep them, and rewrite any whose output
  format this ticket changes so that it asserts the same claim.

## Workflow documentation

Write the exact commands into each ticket that gates on the baseline:

- `plugins-v3-card-class`
- `plugins-v3-builtin-cleanup`
- `plugins-v3-conferral-inlining`
- `plugins-v3-primitive-template-boundary`
- `plugins-v3-builtin-doc-comments`
- `plugins-v3-builtin-spelling-todos`
- `plugins-v3-canon-authoring-surface`
- `plugins-v3-canon-cleanup`

The commands go from saving the baseline under
`docs/evidence/plugins-v3-expansions/<ticket>/` (gitignored), through the change, to comparing,
including the rename-map file each one needs. Update the module doc comment to match.

## Acceptance

- The `bothEach` edit and a bogus identifier in an arm that was never evaluated are both
  reported as changed.
- One invocation saves both the builtin and canon baselines. Comparing an unchanged tree gives
  0 differences and exit 0.
- A 908-entry rename map with no change in meaning reports 908 renames and 0 differences.
- Every gating ticket carries the commands.
- Gates: `cargo test -p xtask -p deckmaste_semantics_v3` and
  `cargo clippy -p xtask -p deckmaste_semantics_v3 --tests -- -D warnings`.
