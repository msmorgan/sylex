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
Cartesian products are eagerly materialized and duplicated across the memo
tables.

## Design decisions

1. **Lazy $k$-best enumeration (Huang & Chiang 2005, Algorithm 3) over an
   internal flyweight arena:**
   - Replace the eager bottom-up `Reading` / `Vec<Reading>` construction in
     `Parse::ranked`, `node_trees`, and `item_sequences` with **Huang & Chiang
     (2005), "Better $k$-best Parsing" (Algorithm 3)** backed by compact `Copy`
     handles into flat arena tables (`TreeEntry` and cons-cell `SeqEntry`).
   - Each forest node and `(item, Anchors)` group maintains its materialized
     best-so-far handle list and a lazy candidate frontier, computing its
     $(k+1)$-th best candidate only when a parent requests it (preserving
     `walk.rs`'s simple-path cycle exclusion when a unary cycle is open on the
     current path).
   - Intermediate nodes and items never allocate `ReadingNode`, `BTreeMap`, or
     `Frame` clones during enumeration.
   - Final tie-break ordering on tied `RankingKey`s compares arena entries
     directly in the exact order of `Reading`'s `Ord` (`production`, `category`,
     `features`, `gap`, `selected_frame`, `children` for `Reading::Node`, and
     `LexicalReading` for `Reading::Leaf`), short-circuiting in $O(1)$ when two
     candidates share the same subtree or prefix handle, so the emitted
     `(RankingKey, Reading)` sequence is identical to the eager implementation.
2. **Iterator API on `Parse`:**
   - Change `Parse::best_readings(k)`, `Parse::best_ranked(k)`, and
     `Parse::readings()` to return iterators that hold the lazy $k$-best state
     and flyweight arena and materialize one owned `Reading` (or `(RankingKey,
     Reading)`) on demand per `.next()` call.
   - Callers that inspect one Reading at a time (`census::judge_parse`,
     `judgment::Judge::evaluate`, and the `english` module's `parse` subcommand
     in `crates/xtask/src/english.rs`) stream without collecting; callers and
     tests that need a slice or set `.collect()`.
3. **Exhaustive round-trip law 2 in the census:**
   - Measure full-corpus census wall time checking round-trip law 2 on every
     Reading (`usize::MAX`) versus the 8-reading census (`LAW_TWO_READINGS = 8`).
   - If exhaustive Law 2 wall time is within **15%** of the 8-reading census
     (and produces 0 Law 2 failures), make exhaustive Law 2 the unconditional
     default and remove `--all-readings`, `LAW_TWO_READINGS`, and
     `Report::law_two_exhaustive`; otherwise keep `LAW_TWO_READINGS = 8` as the
     default with updated documentation.

## Acceptance criteria

- `strata.json` is byte-identical to baseline (aside from removing
  `law_two_exhaustive` if exhaustive Law 2 becomes the unconditional default).
- A temporary differential check across the corpus confirms that for every
  Corpus Unit, the new lazy iterator produces the exact same `Reading` sequence
  as the previous eager implementation.
- Peak memory (RSS) when enumerating all Readings across the corpus
  (`usize::MAX`) is $\le 10\text{ GB}$ (down from ~27 GB).
- Default census CPU and wall time are no worse than baseline.

Standard constraints apply. End the ticket with the two-line landing note from
`docs/tickets/README.md` and update `crates/deckmaste_syntax/README.md` and
`crates/deckmaste_english/README.md` to match.

**Census delta** (`rtuxoqyn`): faces read 32,825 → 32,825; total Readings 326,905 → 326,905; lost faces: none.
**Judgment set** (`rtuxoqyn`): 3,382 enforced, 0 pending; all pass.
