---
needs: []
---

# Exact counts and forks over the Packed Forest

## Why

Ambiguity can be inspected today only by enumerating Readings: `english parse` prints the best
`--limit` of them, and the judge materializes every one (`READING_LIMIT`). The census finds
20,583 of 94,335 Corpus Units with more than one Derivation, and some with thousands (*Remove
Enchantments* 9,216, *Maralen, Fae Ascendant* 5,955, *Draugr Necromancer* 3,312), so a list of
trees cannot show where a unit's analyses differ. `docs/decisions/english.md` §4 asks that
consumers work over the forest and never enumerate every tree. The forest already counts
bottom-up (`Parse::count_node`, `tier_node` in `reading.rs`), and `segments.rs` already shows
that independent stretches multiply (*Remove Enchantments* is 192 × 48). What is missing is the
other direction: how many Derivations of the whole unit use a given node or edge, and from that
the places where the forest offers competing analyses, named in the grammar's own terms.

## Design

- `sylex_syntax`, grammar-agnostic (`english.md` §1): beside the bottom-up count, a top-down
  pass gives every forest node and every edge (a node's (Production, item) family and an item's
  (previous, child) family) its outside count, so inside × outside is the exact number of the
  unit's Derivations that use it.
- A node whose count equals the unit's total is **settled**: it is in every Derivation. Where a
  settled node has one alternative and each of its children is settled, the children vary
  independently, so its Derivations are the product of theirs; a settled node with several
  alternatives is itself a fork.
- `sylex_english`: a **fork** is a place where the forest offers competing analyses, anchored to
  a 1-based word position of the unit (`wN`, counting whitespace-delimited words as `words` in
  `reading.rs` does), the same across calls on the same text and grammar. Each option carries
  its exact count, and the options of a fork sum to the Derivations that reach it.
- A fork is stated through the head and function rules the judgments use
  (`CompiledGrammar::locate`, `head_word`, and `Function` in `crates/english/src/judgment.rs`):
  - attachment: the host (head lemma and word position) and the function borne there;
  - lexical or category: which lexeme, word form or Category a span is;
  - coordination extent: which span a coordination's conjuncts cover;
  - gap antecedent: which constituent a Gap's antecedent is, by the rule of `has_antecedent`;
  - residual: alternatives that differ only in word form, Selected Frame, features or
    Production. A Production-only split whose options yield identical Constituents is flagged
    as a candidate `english.md` §8 duplicate.
- A node's head word and Selected Frame belong to each of its alternatives, not to the node: a
  packed node's families may differ in them.

## Open questions

- Two forks anchored at the same word, and the order of a fork's options: both fix what a
  selector `wN.K` (`english-forestwalk-narrowing`) names. Options: a second index or a kind
  letter in the anchor; options ordered by host word position, by count, or by Production id.
  An order by Ranking Key would read as a recommendation (`english.md` §5).
- Derivations or Readings: the counts here are Derivations, and the enumeration deduplicates two
  Derivations that build the same tree (`Enumeration::get`; glossary, "Derivation"). If the
  cross-check below finds units where the two differ, whether counts are reported in Derivations
  with the gap flagged as an `english.md` §8 defect, or counted per distinct tree.
- Unary cycles: `walk.rs` excludes a node met again on the current path, so below a cycle a
  node's inside count depends on the path that reached it, and inside × outside is not exact
  there. If the pool has such cycles, how outside counts are defined at them.

## Done when

- The claim that a node or an edge occurs at most once in any Derivation (sibling spans are
  disjoint, Empty Productions are refused by the compiler, and unary cycles are excluded) is
  tested on every Corpus Unit of the pool, not assumed; the landing reports whether any pool
  forest has a unary cycle.
- On every unit small enough to enumerate, each node's and edge's count equals the number of
  enumerated Readings that contain it, and the unit's total equals `derivation_count`; the
  landing reports each unit where Derivations and distinct Readings differ.
- On units too large to enumerate, the counts of a fork's options sum to the count of what they
  hang from, and a settled node with one alternative has the product of its children's counts.
- Forks are computed for every unit of the pool without enumerating a Reading, and the landing
  reports the pool's totals by fork kind and the units with the most forks.
- `sylex_syntax` names no grammar Category, feature or function beyond the child names it
  already reads.
- The terms *settled* and *fork* have entries in `docs/contexts/oracle-english/CONTEXT.md`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax` and
  `sylex_english`.

## Not in scope

- A command that prints forks (`english-forestwalk-survey`), narrowing by them
  (`english-forestwalk-narrowing`), the pool database (`english-forest-database`) and the judge
  (`english-judge-reading-limit`).
- Choosing among options: nothing here prefers one analysis (`english.md` §4, §5).

## Pointers

- `docs/decisions/english.md` §1, §4, §5, §8.
- `crates/syntax/src/chart.rs` (`PackedForest`, `ChildRef`), `reading.rs` (`count_node`,
  `count_item`, `tier_node`, `Enumeration`, `locate`, `head_word`, `child_names`, `words`),
  `walk.rs`.
- `crates/english/src/segments.rs`, `crates/english/src/judgment.rs` (`Function`, `holds`,
  `has_antecedent`).
