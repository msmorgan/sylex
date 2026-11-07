---
needs: [idris-retirement]
---
# Delete `lean/`

## Why

Lean is deleted in full and no Lean mirror of the `.syl` language is kept: a mirror would keep
`.syl` bound to Lean's keywords (the source of the trailing-underscore names) and to an AST that
records no written order. Lean stays in jj history, to compare against if a later `.syl` change
breaks something that cannot otherwise be diagnosed. This follows `syl-primitive-constructors`
(checked by Lean's value dump) and the temporary `.syl` resolution validator, which together
guard `.syl` once Lean is gone. The tickets that need this one are then done in `.syl` and Rust
only.

## Done when

- `lean/` is gone, with `sylGen` and the value dump.
- Every Lean-only tool is gone: `cargo xtask lean`, `cargo xtask dsl differential` (it compares
  `sylex_syl` against `SylGen`'s parser), the Lean CI job and its cache in
  `.github/workflows/ci.yml`, the `lean/.lake` seeding in `scripts/provision-workspace`, and the
  `/lean/Generated/` entry in `.gitignore`.
- Nothing tracked names Lean as current: `CLAUDE.md` (the `lean/` line, `lean check` under Tests),
  `docs/decisions/semantics.md` ("Lean is a workbench"), `docs/decisions/syl.md` §2 ("the Lean
  generator"), comments in tracked code such as `crates/syl/tests/parse.rs`, and the memory
  entries that name Lean as current.
- `cite-config.json` no longer scans `.lean` files, and the citation tooling's `.lean` fixtures
  are rewritten.
- The rules cited only in `.lean` files lose their citation. They are listed for the owner, and
  the list is kept so that the Rust AST work (the AST-level rules from `Words.lean` and
  `Abilities.lean`) and the DRT work (the checker and proof rules) cite them again. Coverage is a
  report, not a floor (`cite-coverage-inventory`).
- `cargo test --workspace`, `cargo clippy --workspace --tests -- -D warnings`, `cargo xtask cite
  check` and the `.syl` validator pass.

## Not in scope

Porting Lean's code or doc comments to Rust: the Rust semantics is designed from the ADRs and the
CR, never translated from Lean.

## Pointers

- `lean/`, `crates/xtask/src/lean.rs`, `crates/xtask/src/dsl.rs`,
  `crates/xtask/src/bin/cargo-xtask.rs`
- `.github/workflows/ci.yml` (`lean` job), `scripts/provision-workspace`, `cite-config.json`,
  `crates/xtask/src/coverage.rs`
- `rg -il '\blean\b' --glob '!lean/**' --glob '!docs/tickets/**'` for the remaining mentions
