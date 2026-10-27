---
needs: [plugins-v3-canon]
---
# Key `cargo xtask expansions` output by declaration, so plugin cleanups can prove they preserve meaning

Standard constraints apply. The `plugins-v3-*` cleanups rename, move and rewrite more than 2,000
files in `plugins_v3/`. "It builds and the tests pass" does not catch a declaration or canon card
that still elaborates but now means something else. Each cleanup that preserves meaning must
instead show that every declaration's expansion is unchanged.

`cargo xtask expansions` (`crates/xtask/src/expansions.rs`) already prints each declaration's
expansion, but it writes them to an output tree that mirrors the source paths. A rename therefore
looks like a deletion plus an addition. Change it as follows:

- Add an output mode keyed by `(bare_name, return_type)`, one stable, sorted record per
  declaration that does not depend on file path or position. It must also work for
  `--plugin plugins_v3/canon` (cards keyed by their declaration key).
- Add a comparison mode against an earlier output directory. It lists added, removed and changed
  keys, and exits nonzero on any change unless that key is passed in an explicit `--expect KEY`
  list.
- Document the workflow in the cleanup tickets' terms: take the baseline into the gitignored
  `docs/evidence/plugins-v3-expansions/<ticket>/before/`, make the change, and compare. A
  cleanup that preserves meaning passes with zero changes. A cleanup that renames keys passes
  with an explicit rename map.

## Acceptance

- Running the keyed dump twice on an unchanged tree gives byte-identical output. Renaming a
  builtin file without changing its contents reports no change.
- An `xtask` test covers the comparison mode: an edited body is reported as changed, and an
  `--expect`ed key is accepted.
