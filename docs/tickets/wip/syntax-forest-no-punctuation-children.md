---
needs: []
---

# The Packed Forest stores no punctuation

## Why

A Reading stores no punctuation (`docs/design/english.md` §7), but the Packed Forest it is read
from stores a placeholder for every mark. When the chart moves a Production's dot over a
punctuation symbol it records that step as a child, `ChildRef::Punct`, which says a mark stood
there and not which one; the mark itself is on the Production. Everything that walks the forest
then carries an arm for it: the fork graph (`GChild::Punct`), the enumeration tree
(`Tree::Punct`), the settled spine (`SettledChild::Punct`), the counts, alternatives, ranking,
law and Segment walks, and the forest database, where 172,445 of 3,347,803 `families` rows have
neither a child node nor a leaf. The survey prints each as a line reading `punctuation`: 13 of
them for Soulflayer's second line, ten for the commas of one keyword list. Spelling recovery
reads the forest, so it would meet the placeholders too.

## Done when

- The children of a Constituent in the Packed Forest are Constituents and leaves only, as a
  Reading's are. `ChildRef`, the fork graph's child, the enumeration tree and `SettledChild` have
  no punctuation variant, and no walk of the forest has an arm for one. What the chart keeps to
  cross a mark is its own and is not a child.
- A Production's punctuation is read from the Production wherever it is needed (Realization, the
  round-trip law check, spacing), as it is declared.
- The forest database's `families` has no row without a child node or a leaf, and
  `crates/english/README.md` describes the table as it then is.
- `cargo xtask english forestwalk` prints no line for a mark.
- `docs/design/english.md` §7 says the Packed Forest stores no punctuation, beside the Reading.
- No unit's verdict, Derivation count, Segments, forks or Readings change: `cargo xtask english
  strata --check` passes against a report written before the change, and the landing says how
  the forks were compared.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`,
  `sylex_english` and `xtask`.

## Not in scope

- The punctuation tokens of the profiler (`crates/english/src/profile.rs`), which reads text,
  not the forest.

## Pointers

`crates/syntax/src/chart.rs` (`ChildRef`, `Item::families`, the step over a punctuation symbol),
`crates/syntax/src/fork_graph.rs`, `crates/syntax/src/settled_spine.rs`,
`crates/syntax/src/{counts,alternatives,reading,laws}.rs`, `crates/english/src/segments.rs`,
`crates/xtask/src/english/forests.rs` (the `families` writer),
`crates/xtask/src/english/forestwalk.rs` (`child_text`).
