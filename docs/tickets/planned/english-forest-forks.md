---
needs: [english-forest-counts-and-forks]
---

# Forks: where a unit's analyses differ, read off the forest

## Why

The exact counts (`english-forest-counts-and-forks`) say how many Derivations use each node,
item and edge, but not where a unit's analyses part. A **fork** is meant to be that place:
anchored to a word of the unit, stated in the grammar's own terms (an attachment, a lexical or
Category choice, a coordination's extent, a Gap's antecedent), with each option's exact count.
The survey and narrowing commands and the judge build on it. Measured over the pool's 94,335
units, a fork cannot be read off a forest vertex:

- A vertex with several live families is where analyses re-merge, not where they diverge. Of
  41,555 item vertices with several live families, 34,187 (82%) have options with the same
  previous item and a child of the same Category and span, which differ only further down.
  *Crook of Condemnation*'s "{1}, {T}: Exile target card from a graveyard." has 2 Derivations
  and one such vertex, a settled item at w3 whose options are two Constituents of one Category
  over "Exile target card from a graveyard"; the difference, whether "from a graveyard" modifies
  "card" or adjoins the Verb Phrase, lies five levels below.
- 21,537 node vertices also have several live families.
- The root set is a fork of its own: 83 units have more than one live root (81 with 2, 2 with
  3), which no vertex holds.
- The function rules the judgments use (`locate`, which is private; `child_names`,
  crate-private; `head_word`) work on a Reading, not on the forest.
- Two anchor rules were measured over the vertex forks. Anchoring at the word where the item
  starts puts 31,857 forks at a word another fork shares, in 6,628 units, up to 40 at one word;
  anchoring at the first word of the option's child puts 27,329 there, in 5,784 units, up to 25
  at one word (338 of them settled). 254 forks have no word to anchor at under either rule.

## Design

- A fork is anchored to a 1-based word position of the unit (`wN`, counting
  whitespace-delimited words as `words` in `reading.rs` does), the same across calls on the same
  text and grammar. Each option carries its exact count in Derivations, and the options of a
  fork sum to the Derivations that reach it.
- A node's head word and Selected Frame belong to each of its alternatives, not to the node.
- Forks are computed without enumerating a Reading, in `sylex_english` over the counts of
  `sylex_syntax`.

## Open questions

- How two options are compared downward to find where they differ: the vertex holds them
  together, but the analyses part only below it, possibly at several places.
- Which word anchors a fork, given the collisions measured above.
- How two forks at one word are told apart, and how a fork's options are ordered (both fix what
  a selector `wN.K` of `english-forestwalk-narrowing` names). Never by count or by Ranking Key,
  which would read as a recommendation (`english.md` §5).
- How the function rules (`locate`, `head_word`, `child_names`, `has_antecedent`) are shared
  between Readings and the forest, so that a fork is stated in the same terms as a judgment.

## Done when

- Forks are computed for every unit of the pool without enumerating a Reading, and the landing
  reports the pool's totals by fork kind and the units with the most forks.
- On every unit small enough to enumerate, each fork's option counts equal the number of
  enumerated Readings that take that option.
- The term *fork* has an entry in `docs/contexts/oracle-english/CONTEXT.md`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- Printing forks (`english-forestwalk-survey`), narrowing by them
  (`english-forestwalk-narrowing`) and storing them (`english-forest-database-forks`).
- Choosing among options (`english.md` §4, §5).

## Pointers

- `docs/decisions/english.md` §1, §4, §5, §8.
- `crates/syntax/src/reading.rs` (`locate`, `head_word`, `child_names`, `words`), the outside
  pass of `english-forest-counts-and-forks`.
- `crates/english/src/judgment.rs` (`Function`, `holds`, `has_antecedent`).
- The research probe in the gitignored `docs/evidence/english-forest-counts-and-forks/`.
