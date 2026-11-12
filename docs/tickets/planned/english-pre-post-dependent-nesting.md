---
needs: [english-forest-forks]
---

# A pre-head and a post-head dependent nest in either order

## Why

Where a head has one dependent before it and one after it, the grammar builds both nestings:
the pre-head dependent outside (*it also [gets +3/+0 until end of turn]*) and inside (*it [also
gets +3/+0] until end of turn*). The two Readings answer every fork of `english-forest-forks`
alike (each dependent bears the same function in a phrase of the same Category and head word),
so no fork tells them apart. On the pool, 112 texts have two Readings that first differ there
in a Verb Phrase, a central adverb against a post-head Adjunct, and 11 in a Nominal, a
coordinated premodifier against a postmodifier. With *Heart of Bogardan*
(`english-multiplier-postmodifier-scope`) they are all 124 texts whose Readings the forks do not
separate:

- *Goatnap*: "it also gets +3/+0 until end of turn"; *Might of the Meek*: "It also gets +1/+0
  until end of turn if you control a Mouse" (3 Readings);
- *The Clone Saga*: "When you next cast a creature spell this turn";
- *The Eternal Wanderer*: "creatures they control not chosen this way";
- *Master of Cruelties*: "This creature can only attack alone";
- *Bloodthirster*: "a player it has already attacked this turn";
- *Lock and Load*: "each other instant and sorcery spell you've cast this turn".

`cargo test --release -p sylex_english --test forest_forks
forks_agree_with_enumerated_readings_on_the_pool -- --ignored --nocapture 2>&1 | grep
'^UNSEPARATED' | grep -E 'CentralAdverbial|CoordinationPremodifier'` lists them with the
Productions where two such Readings first differ.

## Open questions

- Which of these pairs are two trees of one analysis (`docs/decisions/english.md` §8), to be
  canonicalized in the chart, and which differ in meaning (*not [chosen this way]* against *[not
  chosen] this way*, *only [attack alone]*, *next [cast a creature spell this turn]*).
- For pairs that differ in meaning, whether a fork kind tells them apart (which of two
  dependents of one head is nearer it), or the grammar marks the difference otherwise.

## Done when

- Every text the command lists has one Reading per analysis for its pre-head and post-head pair,
  or its Readings differ in some fork.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `docs/decisions/english.md` §8; `crates/syntax/src/normal_form.rs` (the fronted Clause
  adjunct, kept outermost).
- `crates/syntax/src/forks.rs`; `crates/english/tests/forest_forks.rs`.
