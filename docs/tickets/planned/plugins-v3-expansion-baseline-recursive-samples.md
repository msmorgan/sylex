---
needs: []
---
# Sample recursive constructors in the expansion baseline

Standard constraints apply. `constructor_variants` in
`crates/deckmaste_semantics_v3/src/lower/snapshot.rs` builds a sample with the constructor's
own type in the exclusion set. A constructor with a field of its own type therefore fails, and
the constructor is dropped from sampling. The summary does report these drops: 64 in the
`plugins-v3-builtin-doc-comments` run, among them `Action.doIf`, `Action.delay`,
`Ability.qualified`, `Amount.arith` and `Amount.half`.

The baseline therefore misses edits in any `match` arm on those constructors, the same kind of
miss as the original `bothEach` defect. Every later cleanup gates on this baseline.

## Change

- Sample recursive constructors at depth 1, filling each self-typed field with that type's
  base sample. Report any constructor that still cannot be sampled, and the reason, as now.
- `Option some` of a non-finite type, and fields nested inside constructor samples, currently
  get only the base sample. Give them the same depth-1 treatment, or list what stays uncovered
  in the module doc comment.
- The sample cap of 256 stays. The summary already reports what the cap leaves out.

## Acceptance

- The omitted-recursive-constructor count drops to the cases that genuinely cannot be sampled,
  each listed with its reason.
- Regression test: an edit in a `match` arm on `Action.doIf` in a copied builtin template is
  reported as changed.
- Comparing the current tree against a fresh baseline gives 0 differences.
- Gates: `cargo test` and `cargo clippy --tests -- -D warnings` for `deckmaste_semantics_v3`
  and `xtask`.
