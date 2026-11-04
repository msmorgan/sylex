---
needs: [lean-retirement]
---
# Complete `.syl` spellings for non-parameter-order declarations and drop `Action::DoIf.postposed`

Standard constraints apply. Follow-up after `lean-retirement`: once the AST is unfrozen and
`spelling` syntax supports optional clauses, replace the `read_order` seeds in
`deckmaste_semantics_v3` with `.syl` `spelling` strings (or reordered parameters if the
optional-argument naming rule is adopted) and delete the frozen-AST `Action::DoIf.postposed` field.

- Add `spelling` strings (or align parameter order) for every declaration in `plugins/builtin/`
  whose printed argument order differs from its parameter declaration order and currently carries
  `# TODO(spelling)` (`when`, `whenever`, `at`, `triggered`, `gatedBy`, `staticSpecGatedBy`), then
  remove the `read_order` seeds in `crates/deckmaste_semantics_v3`. The overrides
  (`ReadOrder.fieldOrders`, where resolution steps arguments in an order other than the printed one)
  stay.
- Delete `Action::DoIf.postposed` from `crates/deckmaste_semantics_v3` and
  `plugins/builtin/templates/action/doOnlyIf.syl`.
