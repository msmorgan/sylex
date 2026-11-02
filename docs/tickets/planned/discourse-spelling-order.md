---
needs: [lean-retirement]
---
# Complete `.syl` spellings for non-parameter-order declarations and drop `Action::DoIf.postposed`

Standard constraints apply. Follow-up after `lean-retirement`: once the AST is unfrozen and
`spelling` syntax supports optional clauses, replace the seeded `spelling_order` overrides in
`semantics` with `.syl` `spelling` strings (or reordered parameters under the settled optional-argument
naming rule) and delete the frozen-AST `Action::DoIf.postposed` field.

- Add `spelling` strings (or align parameter order) for every declaration in `plugins_v3/builtin/`
  whose printed argument order differs from its parameter declaration order and currently carries
  `# TODO(spelling)` (`doOnlyIf`, `when`, `whenever`, `at`, `gatedBy`, and the `ReadOrder.fieldOrders`
  primitives), then remove the seeded order overrides in `crates/semantics`.
- Delete `Action::DoIf.postposed` from `crates/semantics` and `plugins_v3/builtin/templates/action/doOnlyIf.syl`.
