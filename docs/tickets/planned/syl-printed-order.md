---
needs: [semantics-rust-crate]
---
# Stage 2 of `semantics-rust-rewrite`: write every call in printed order

Standard constraints apply. With interleaved arguments accepted (`dsl-named-arguments-anywhere`)
and an expander to check against (`semantics-rust-crate`), reorder the canon and builtin calls
whose named arguments have a printed position, so the written order of a `.syl` call is the
printed order of its text: the intervening-"if" triggers first (`when`, `whenever`, `at` with
`if =`), then any other call whose arguments are written out of printed order.

- Reordering changes no meaning. Gate: every touched file expands to the same core AST in
  `semantics` and in Lean before and after (Lean sorts arguments into binder order, so its
  checker is unaffected).
- `doOnlyIf` and `doIf`'s `postposed` flag stay as they are: they encode timing as well as order
  and are replaced in `discourse` (`semantics-rust-rewrite`), not here.
- Done before `discourse` is built, so `discourse` never ports the per-constructor rules that
  recover printed order (`lean/SylGen/ReadOrder.lean`'s field orders, and the checker arms that
  step an intervening condition before its action).
