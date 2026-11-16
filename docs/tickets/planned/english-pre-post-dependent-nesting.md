---
needs: [english-multiplier-noun-phrase-productions]
---

# Canonical nesting for pre-head and post-head dependents

## Why

Where a head has one dependent before it and one after it, the grammar builds both nestings:
the pre-head dependent outside (*it also [gets +3/+0 until end of turn]*) and inside (*it [also
gets +3/+0] until end of turn*). The two Readings answer every fork alike (each dependent bears the
same function in a phrase of the same Category and head word), so no fork tells them apart. On the
pool, 112 texts have two Readings that first differ there in a Verb Phrase (a central adverb
against a post-head Adjunct), 11 in a Nominal (a coordinated premodifier against a postmodifier),
and 1 in a Noun Phrase (*Heart of Bogardan*: `"where X is twice the number of age counters on this
enchantment minus 2"`, where `MultipliedNounPhrase` and `NounPhraseArithmetic` commute across
`"number"`):

- *Goatnap*: "it also gets +3/+0 until end of turn"; *Might of the Meek*: "It also gets +1/+0
  until end of turn if you control a Mouse" (3 Readings);
- *The Clone Saga*: "When you next cast a creature spell this turn";
- *The Eternal Wanderer*: "creatures they control not chosen this way";
- *Master of Cruelties*: "This creature can only attack alone";
- *Bloodthirster*: "a player it has already attacked this turn";
- *Lock and Load*: "each other instant and sorcery spell you've cast this turn";
- *Heart of Bogardan*: "where X is twice the number of age counters on this enchantment minus 2".

`cargo test --release -p sylex_english --test forest_forks
forks_agree_with_enumerated_readings_on_the_pool -- --ignored --nocapture 2>&1 | grep
'^UNSEPARATED'` lists them with the Productions where two such Readings first differ.

## Design

- Canonicalize one nesting order per category in `english.fug` (`docs/design/english.md` §8), with
  no new fork kind in `forks.rs`:
  - `VerbPhrase`: `CentralAdverbial` sits outermost over post-head VP adjuncts.
  - `Nominal`: postmodifiers (`NominalPostmodifier`, `PostpositiveAdjective`, `Adjunct:Nominal`
    Preposition Phrases, `CoordinatedPostmodifier`, `TemporalPostmodifier`) sit outermost over
    `CoordinationPremodifier` (matching single-head `NominalPostmodifier(Unmodified(Premodifier(...)))`).
  - `NounPhrase`: `NounPhraseArithmetic` (*plus*, *minus*) sits outermost over
    `MultipliedNounPhrase`, giving *Heart of Bogardan* `[[twice the number of age counters on this
    enchantment] minus 2]` (`(2 * N) - 2`).

## Done when

- Every text the `forest_forks` check lists has one canonical Reading for its pre-head and
  post-head pair, including *Heart of Bogardan*'s `[[twice …] minus 2]`, and zero unseparated
  pairs remain.
- `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass
  for each changed crate.

## Pointers

- `crates/english/grammar/english.fug`: `CentralAdverbial`, `CoordinationPremodifier`,
  `NominalPostmodifier`, `MultipliedNounPhrase`, `NounPhraseArithmetic`.
- `docs/design/english.md` §8; `crates/syntax/src/normal_form.rs` (the fronted Clause
  adjunct, kept outermost).
- `crates/syntax/src/forks.rs`; `crates/english/tests/forest_forks.rs`.
