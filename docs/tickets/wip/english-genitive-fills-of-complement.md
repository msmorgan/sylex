---
needs: []
---

# A genitive and an *of* Complement fill one argument twice

## Why

CGEL counts a genitive subject-determiner among an NP's complements: *the flood's consequences*
and *the consequences of the flood* each have one complement, the cause, in two forms (Ch. 5,
§16.5.3, p. 475, [56ii]). A noun whose *of* frame names the one argument a genitive can name
(*hand*, *library*, *graveyard*, *life total*, *mana value*, *name*, *ability*) can therefore take
one or the other, never both, yet the grammar lets a genitive-determined head take its *of*
Complement too. Right Node Raising then shares an *of* phrase onto it: in One with the Multiverse
line 3, *your hand or the top of your library* reads *[your hand or the top] of your library* (5
of 13 Derivations), and in Wheel of Potential line 2 *exile their hand and draw a number of cards*
shares *of cards* onto *their hand* (30 of 198 Derivations). Both are second trees no English
reader builds.

## Done when

- A head whose genitive determiner names the argument its *of* frame names takes no *of*
  Complement, directly or shared, by a rule whose comment cites CGEL; a noun with two arguments
  (*Van Dyck's portrait of Charles I*, p. 475, [56i]) is not affected.
- One with the Multiverse line 3 and Wheel of Potential line 2 lose those Readings; the census
  shows which Readings changed and why, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

`crates/vocabulary/lexicon/english/nouns.ron`; `crates/english/grammar/english.fug`
(`valence BareNominal`, the genitive determiner productions); `crates/syntax/src/compile.rs`
(the `gap0` noun Complement variants);
`cargo xtask english forestwalk --card "One with the Multiverse" --line 3`.
