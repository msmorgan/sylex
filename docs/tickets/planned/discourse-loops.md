---
needs: [discourse-outcomes]
---
# `discourse-loops`: loops, plural abstraction, aggregates, and partitives

Standard constraints apply. Loop, aggregate, and partitive family of `semantics-rust-rewrite`:
implement `doForEach` plural abstraction ($\Sigma X$), loop result publication, `countOf` /
`aggregate` reductions, partitives, `theRest` / `theOther`, slice selections, and pile partitions
in `crates/deckmaste_discourse`.

## Scope & Design Decisions

1. **Loops and plural abstraction (`LoopResults.lean`, `Action::DoForEach`)**:
   - Step `Action::DoForEach { group, body }` by binding the singular member referent for `body`
     (supporting both object/player domains and event/outcome domains).
   - On loop exit, abstract the body's per-iteration results into plural `LoopAggregate` referents
     in the outer `DRS` (`publishLoopTargetResults`, `publishLoopCreatedResults`,
     `publishLoopOutcomes`) so post-loop clauses resolve `"those creatures"`, `"those tokens"`, and
     `"that much"` / `"the total amount"` across the whole iteration.

2. **Counts, aggregations, and comparisons (`Amount::CountOf`, `Amount::Aggregate`)**:
   - Check `Amount::CountOf` (`forEach`) and `Amount::Aggregate` (`total`, `greatest`, `least`,
     `distinctCount` over `ProjAxis`) and bound-member comparisons
     (`lean-v3-compare-over-bound-reads-member`).

3. **Partitives, `theRest`, `theOther`, slices, and piles (`Selections.lean`)**:
   - Implement partitive noun phrases (`Quantity` over a base `NounPhrase`), requiring a plural
     base (`Refusal::PartitiveBase`).
   - Implement `NounPhrase::TheRest` and `NounPhrase::TheOther` complement resolution against a
     prior subset selection from the same base group (`lean-v3-partitive-rest-reads-its-kind`).
   - Implement slice selections (`top`, `bottom`, `random` of a zone/group) and pile partitions
     (`separateIntoPiles`, `chosenPile`, `otherPile`).

## Tests & Verification

- **Corpus & fixture gate**: Every corpus card and fixture with no remaining `Refusal::Unsupported`
  agrees with `bindings.jsonl` and `fixtures.jsonl` (plus the documented fixes).
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 2 Suites H (Zones, piles, votes,
  off-stack) and L (Descriptions, quantity grammar) as native Rust tests.
