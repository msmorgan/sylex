---
needs: [dsl-parser, dsl-parse-differential, plugins-v3-data-language, lean-proofs-spec-catalog, dsl-named-arguments-anywhere, dsl-plain-word-names]
---
# [design] Stage 1 of `semantics-rust-rewrite`: the `semantics` crate

Standard constraints apply. The first, cheaper half of the rewrite (owner, 2026-11-02): resolve
names by expected sort (semantics-v3 §4.5) and expand Templates and Definitions over the `.syl`
syntax tree `deckmaste_dsl` parses, with no reference resolution. It is built before the corpus is
reordered (`syl-printed-order`), so that reordering is checked by an expander that already
exists, and before `discourse`, which reads its output.

- Calls with interleaved named and positional arguments resolve as `deckmaste_dsl` parses them
  (`dsl-named-arguments-anywhere`).
- The resolved tree keeps the order in which a call's arguments were written: `discourse` steps
  them in that order (`semantics-rust-rewrite`).
- Verification: per declaration and card, `semantics`' expansion equals the value Lean elaborates
  (`lean-elaborated-value-dump`), up to that recorded argument order.
- Lean stays in the tree and in CI throughout.
