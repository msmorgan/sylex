---
needs: [english-forest-forks]
---

# A multiplier against a post-nominal modifier: two meanings no fork tells apart

## Why

*Heart of Bogardan*'s "X is twice the number of age counters on this enchantment minus 2"
has Readings with *minus 2* inside the multiplied Noun Phrase (`AdverbMultipliedNounPhrase`
over `NounPhraseArithmetic`: 2(N−2)) and outside it (`NounPhraseArithmetic` over
`AdverbMultipliedNounPhrase`: 2N−2). The two mean different numbers, yet they answer every fork
of `english-forest-forks` alike: both bracketings lie on *number*'s head line, so *twice* and
*minus 2* each bear the same function in a Noun Phrase headed by *number* in both. It is the
one such unit in the pool; `cargo test --release -p sylex_english --test forest_forks
forks_agree_with_enumerated_readings_on_the_pool -- --ignored --nocapture 2>&1 | grep
'^UNSEPARATED' | grep AdverbMultipliedNounPhrase` lists it. *Pox*'s "a third of the cards in
their hand" is not one: "in their hand" modifies "cards", modifies "a third of the cards" or
adjoins the verb, three governors that a dependency fork tells apart.

## Open questions

- Whether the multiplier heads its phrase (so that *minus 2* depends on *twice* or on
  *number*, a dependency fork), whether forks gain a question that tells the bracketings apart,
  or whether the grammar builds only one of them.

## Done when

- *Heart of Bogardan*'s two bracketings differ in some fork, or the grammar builds one
  Reading per meaning and the forks tell those apart.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/english/grammar/english.fug` (`AdverbMultipliedNounPhrase`, `NounPhraseArithmetic`).
- `crates/syntax/src/forks.rs`; `crates/english/tests/forest_forks.rs`.
