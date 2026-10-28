---
needs: [plugins-v3-canon-authoring-surface]
---
# Eliminate the qualified sites retained by builtin cleanup

Standard constraints apply.

Remove the remaining `Semantics.` qualifications retained in Step 2 of
`plugins-v3-builtin-cleanup`. Change `ouqkwnvy` lists all 102 original sites; the cleanup
ticket summarizes their causes. Reconcile that inventory with the current declarations.

Fix the underlying obstacles before shortening the sites:

- Untyped Lean bundle-DAG cycle false positives on `Semantics.Definitions.*` and
  `Semantics.CardType.*`, preserving authored calls, deed references and parameter identity.
- The `.sets` multi-token argument check in
  `crates/deckmaste_semantics_v3/src/lower/compute.rs:495` for
  `Semantics.Characteristics.valueEdits`, including the typed conditional in
  `copyCharacteristics`.
- Qualified helper/projection dispatch in templates (`Characteristics.*`, `Cost.asPaid`,
  `Location.observedOnDeparture`, `QualityPayload.edits`) and qualified pattern heads.

Use the settled canon authoring surface, preserving helper behavior and pattern matching.
Require unchanged builtin and canon expansions, clean Lean checking, and focused Rust and
Lean regressions for dependency resolution and helper argument parsing. Kept qualifications
are defects to eliminate, not permanent exceptions.
