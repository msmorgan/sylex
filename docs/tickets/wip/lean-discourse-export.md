---
needs: []
---
# Export Lean's discourse bindings and closed checks as an oracle for `discourse`

Standard constraints apply. The Lean side of `semantics-rust-rewrite`'s verification: a
development tool outside the compile path, deleted with Lean (`lean-retirement`).

- **Per-card bindings.** For every card, the bindings Lean's checker produces: first and later
  mentions, `D′`, `AC>R1` choices. A mention and its antecedent are keyed by their expanded core
  AST path, a sequence of (constructor, field, list index) from the card root, plus a role tag for
  what that node introduces (`Intro(k)` for its k-th referent, `Outcome(sort)`, `MovedSuccessor`,
  `LoopAggregate`, …). Not surface parameter names, argument positions or referent ids: list
  elements, a Definition parameter substituted at several sites, and antecedents that are no
  argument (created tokens, outcomes, piles, mana letters, zone-move successors) all need it, and
  ids depend on stepping order.
- **Fixture snapshot.** The closed `*.check … = …` equations in `lean/Semantics/Proofs/`, evaluated
  into (input, expected) pairs that Rust runs as one data-driven test. A fixture compares accepted
  against refused and the set of refusal kinds, never the exact refusal list: matching Lean's
  multiplicity and order would carry its plumbing into Rust.
- Keep the exporter disposable: it handles the plain equations over the public entry points
  (`Card.check`, `Ability.check`, `Action.check`, the binding results) and lists the names of every
  other theorem (statements with `∧`, `≠`, `.all` or `let`, and those over checker internals such
  as `stepAction` or `causedEvents`); `docs/semantics-spec-catalog.md` already covers those.
- Entries the spec catalog marks FLAG DEFECT (`discourse-parameter-argument-resolution`) are
  marked in the export, since `discourse` is expected to differ there.
- The export format is settled with `semantics-rust-rewrite`'s design dialogue if it is still open
  then; the keying above is not open.
