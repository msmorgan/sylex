---
needs: [english-forest-counts-and-forks]
---

# Judgments checked on units of any size

## Why

`READING_LIMIT` is 4096 (`crates/english/src/judgment.rs`). `Judge::evaluate_scoped` fails an
entry outright above it ("derivations, over the reading limit") and below it materializes every
Reading, checks round-trip law 2 on each and locates each one's Constituents before testing a
single assertion. Two units of the pool are over the limit today (*Remove Enchantments*, 9,216;
*Maralen, Fae Ascendant*, 5,955), so neither can carry a judgment. The census has the same limit
in another form: `judge_parse` in `census.rs` looks for an endorsed Reading among the best 4096
only, so a unit over the limit whose endorsed Reading ranks lower is reported known-wrong-only.
That is a silent truncation, and it makes the census depend on rank, which
`docs/decisions/english.md` §5 forbids. Every assertion names a Constituent by Category, surface,
head, function and Gap antecedent, so the forest can be pruned to the Derivations that could
satisfy it before any tree is built, with the counts of `english-forest-counts-and-forks`.

## Design

- An entry's assertions are checked on the forest: each group of assertions prunes the forest to
  the Derivations that satisfy all of them, and the group holds when the pruned count is
  nonzero; a negated assertion holds when the Derivations satisfying it count zero; `readings
  N` compares N with the exact count.
- `READING_LIMIT` and the census's best-4096 search go.

## Open questions

- Round-trip law 2: `evaluate_scoped` checks it on every Reading, while the census's module doc
  says law 2 follows from law 1 for parser-produced Readings, so no per-Reading pass is needed.
  Options: drop the per-Reading law 2 check from the judge, as the census does; keep it for units
  small enough to enumerate and skip it above that; or check it on a bounded sample.
- The function and Gap antecedent rules are not local to one node: `locate` passes a function
  Category's function down to its head and adds the `also` functions of clusters, Right Node
  Raising and postposed modifiers, and `has_antecedent` climbs to the Gap's binder. Whether the
  forest check restates these rules over the forest or applies them to a bounded enumeration of
  the pruned forest, with the pruning on Category, surface and head only.
- `readings N` counts distinct Readings today (`located.len()`); the exact count is of
  Derivations. Settled with the same open question of `english-forest-counts-and-forks`.

## Done when

- `READING_LIMIT` no longer exists, and nothing in `judgment.rs` or `census.rs` enumerates
  Readings to decide whether an assertion holds or which verdict a unit gets; the law 2 check
  is as the open question settles it.
- On every unit of the judgment set, the forest check and the enumerating check agree, by a
  differential test run once in the landing.
- A judgment on *Remove Enchantments*' line and one on *Maralen, Fae Ascendant*'s third line,
  each asserting its endorsed analysis, hold.
- `cargo xtask english strata --check` passes, and the landing reports any unit whose verdict
  changed, with the reason.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_english` and `xtask`.

## Not in scope

- The judgment entry syntax.
- Ranking: where the endorsed Reading ranks stays reported, never gated (`english.md` §5).

## Pointers

- `crates/english/src/judgment.rs` (`READING_LIMIT`, `Judge::evaluate_scoped`, `holds`,
  `has_antecedent`), `crates/english/src/census.rs` (module doc, `judge_parse`).
- `crates/syntax/src/reading.rs` (`locate`, `Constituent`, `best_readings`).
- `crates/english/judgments/README.md`, `docs/decisions/english.md` §4, §5.
