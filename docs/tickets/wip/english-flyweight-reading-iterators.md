---
needs: []
---
# Flyweight Reading enumeration and on-demand Reading iterators

`Parse::ranked` (`crates/deckmaste_syntax/src/reading.rs`) currently constructs
and deep-clones owned `Reading` trees—each allocating a `BTreeMap<Ident, Ident>`
for `features`, cloning `Option<Frame>`, and cloning `Vec<Reading>` child
subtrees—at every intermediate node and Earley item in the `PackedForest`
(`TreeMemo` and per-node/per-item `BTreeSet`s) before returning a `Vec`. Even
with the worst-case Corpus Unit (*Remove Enchantments*) down to 9,600
Derivations and the whole corpus at ~328k Derivations, running the census with
`usize::MAX` (`cargo xtask english strata --all-readings`) peaks at ~27 GB of
memory across worker threads because intermediate subtrees and cross-Segment
Cartesian products are duplicated across the memo tables.

## Design decisions

1. **Internal flyweight arena during `Parse::ranked`:**
   - Replace the owned `Reading` / `Vec<Reading>` entries in `TreeMemo`,
     `node_trees`, and `item_sequences` with compact `Copy` handles into flat
     arena tables built over the `Parse` (such as `TreeEntry { node, production,
     children }` and cons-cell `SeqEntry { prev, last }`).
   - Intermediate nodes and items never allocate `ReadingNode`, `BTreeMap`, or
     `Frame` clones during the bottom-up walk.
   - Final tie-break ordering on tied `RankingKey`s compares arena entries
     directly, short-circuiting in $O(1)$ when two candidates share the same
     subtree or prefix handle, while preserving the exact `(RankingKey,
     Reading)` order.
2. **Iterator API on `Parse`:**
   - Change `Parse::best_readings(k)`, `Parse::best_ranked(k)`, and
     `Parse::readings()` to return iterators that hold the compact flyweight
     arena and materialize one owned `Reading` (or `(RankingKey, Reading)`) on
     demand per `.next()` call.
   - Callers that inspect one Reading at a time (`census::judge_parse`,
     `judgment::Judge::evaluate`, `xtask::english::parse`) stream without
     collecting; callers and tests that need a slice or set `.collect()`.
3. **Exhaustive round-trip law 2 in the census:**
   - Measure full-corpus census runtime checking round-trip law 2 on every
     Reading by default (`usize::MAX`). If the runtime is fast enough for normal
     census runs, check all Readings by default and remove `--all-readings`,
     `LAW_TWO_READINGS`, and `Report::law_two_exhaustive`; otherwise keep
     `LAW_TWO_READINGS = 8` as the default with updated documentation.

Standard constraints apply. End the ticket with the two-line landing note from
`docs/tickets/README.md` and update `crates/deckmaste_syntax/README.md` and
`crates/deckmaste_english/README.md` to match.
