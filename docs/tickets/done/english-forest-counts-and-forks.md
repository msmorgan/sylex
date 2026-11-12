---
needs: []
---

# Exact Derivation counts over the Packed Forest

## Why

Ambiguity can be inspected today only by enumerating Readings: `english parse` prints the best
`--limit` of them, and the judge materializes every one (`READING_LIMIT`). The census finds
20,583 of 94,335 Corpus Units with more than one Derivation, and some with thousands (*Remove
Enchantments* 9,216, *Maralen, Fae Ascendant* 5,955, *Draugr Necromancer* 3,312).
`docs/decisions/english.md` §4 asks that consumers work over the forest and never enumerate every
tree. The forest already counts bottom-up (`Parse::count_node`). What is missing is the other
direction: how many Derivations of the whole unit use a given node, item or edge. The forks
ticket (`english-forest-forks`), the pool database and the judge's limit build on that.

## Design

- `sylex_syntax`, grammar-agnostic (`english.md` §1): beside the bottom-up inside count, a
  top-down outside pass gives every reachable node, item and edge (a node's (Production, item)
  family and an item's (previous, child) family) its outside count, so inside × outside is the
  exact number of the unit's Derivations that use it.
- A node whose count equals the unit's total is **settled**: it is in every Derivation. A
  settled node with several alternatives is itself a point of choice, so its children vary
  independently only within one alternative.
- The counts are of Derivations. They equal the unit's distinct Readings, and a unit where the
  two differ is an `english.md` §8 defect (two trees of one analysis), caught by a test.
- The counts rest on every node, item and edge occurring at most once in a Derivation, which
  needs two premises:
  - Nothing spans nothing: no Constituent, and no partial Production past its first child. The
    compiler refuses a Production whose first child is a period (a period may be unwritten and
    match with zero width), and the chart asserts that every lexical alternative spans at least
    one char.
  - The forest has no unary cycle. Grammars may contain one: the outside pass checks the unit's
    reachable forest and fails with an error naming the cycle's Categories, spans and
    Productions, never a wrong count. No pool forest has one. The grammar's one Category-level
    unary self-loop, `RelativeAdjunctGap`, cannot close, because a Production that introduces
    a Gap refuses a child that already has one.

## Done when

- The outside pass and the settled flag are public in `sylex_syntax`, small and documented, and
  name no grammar Category, feature or function.
- On real units of every size class, including *Remove Enchantments* (9,216) and *Draugr
  Necromancer* (3,312), each node's, item's and edge's count equals the unit's total minus a
  recount without it, and on units small enough to enumerate it equals the number of
  enumerated Derivations that contain it; the same cross-check over the whole pool is available
  as an opt-in run.
- A test fails, naming the unit, if any unit's Derivation count differs from its count of
  distinct Readings.
- The outside pass on a forest with a unary cycle returns an error naming the cycle, tested on
  a cyclic grammar.
- The term *settled* has an entry in `docs/contexts/oracle-english/CONTEXT.md`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax` and
  `sylex_english`, and the English strata run shows no unit changed its outcome.

## Not in scope

- Forks, and anything that names where analyses differ: `english-forest-forks`.
- Choosing among analyses: nothing here prefers one (`english.md` §4, §5).

## Pointers

- `docs/decisions/english.md` §1, §4, §8.
- `crates/syntax/src/chart.rs` (`PackedForest`, `ChildRef`), `reading.rs` (`count_node`,
  `count_item`, `Enumeration`), `walk.rs`, `compile.rs`.
- `crates/syntax/tests/contract.rs` (obligations 3, 4 and 9).
