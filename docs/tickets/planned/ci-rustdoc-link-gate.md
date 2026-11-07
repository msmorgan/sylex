---
needs: []
---

# Make `cargo doc` a gate: escape bracketed citations and placeholders

## Why

A broken intra-doc link is invisible to clippy and the tests; only rustdoc reports it, and no CI
step runs rustdoc, so deleting a type can leave dead doc links behind a green run. The check is
unusable today because doc-comment citations in the `[CR#…]` format parse as links to a
nonexistent item `CR`. Over the `crates/` packages, `cargo doc --no-deps` emits about 1,690
warnings, about 1,600 of them for `CR`, and they bury the real ones.

## Done when

- Every `[CR#` in a `///` or `//!` doc comment under `crates/` is written `\[CR#` (opening bracket
  only: an escaped closing bracket puts the backslash into the rule number `cargo xtask cite`
  reads). Plain `//` comments and non-Rust files are untouched.
- Bracketed placeholders in doc prose (`[2]`, `[Vampire]`, `[,]`, `[who]`) get the same
  opening-bracket escape, and code-like or HTML-like text (`<effect>`) is in backticks.
- Every remaining dead link (for example `Predicate::r#type`, `SubtypeRef`,
  `DeonticAction::Cast`, `VerbName`, `sylex_plugin_v1::Strategy`) and every public doc linking a
  private item (for example `discard_body_whose` to `hand_owner_ref`) is fixed.
- `cargo doc --no-deps` over the `crates/` packages emits no warnings and runs in
  `.github/workflows/ci.yml`.
- The set of cited rules is unchanged by the sweep.
- `CLAUDE.md`'s citation rule states the escaped form for doc comments.

## Not in scope

- The `v1/` crates: they are not swept and the gate does not document them.
- The citation format anywhere but Rust doc comments.
- A blanket `#![allow(rustdoc::broken_intra_doc_links)]`, which hides what the gate exists to show.

## Pointers

- `crates/xtask/src/cite.rs` (`bracketed_refs`, `blank_bracketed_citations`),
  `crates/xtask/src/coverage.rs` (`extract_bracket_rules`)
- `.github/workflows/ci.yml`, `CLAUDE.md` (CR citations)
