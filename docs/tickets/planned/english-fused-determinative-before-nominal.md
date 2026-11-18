---
needs: [english-frames-held-in-grammar]
---
# A fused determinative before a word it could determine

## Why

A determinative that stands alone as a fused determiner-head (*each enters with ...*) is also
read as one right before a word it could simply determine. `NonNumericComplement` refuses that
for *each* by banning fused *each* from every Complement, which is wider than the defect: it also
refuses *of each*, so *create one of each* (Academy Manufactor) needs the flat Production
`QuantityPartitiveOfEach`, and `NominalPredicative` carries a like exclusion. Without the ban,
seven units gain a second tree:

- *Activate [no more than [twice each]] turn* (Pit Imp, Vampire Bats, Phyrexian Battleflies,
  Fire-Belly Changeling, Roterothopter).
- *[pays {X} for each] [attacking creature they control]* (War Tax).
- *[Double the life stakes with each] flip* (Game of Chaos).

The following word is the head in the first, a determinative homograph that opens another Noun
Phrase in the second and a verb homograph in the third, so no feature equation on the fused
phrase or its host states the refusal. The other fused determinatives (`class FusedQuantifier`,
`class FusedChoice`) have no ban.

## Open questions

- What the refusal is. One candidate: a determinative is no fused head where the same word
  determines a Nominal in some Derivation of the unit, decided in the chart with the grammar's
  own agreement. Another: an adjacency rule like the bare head's omitted Complement, on the next
  word's spellings. Each needs counterexamples sought among Subjects (*if both attack*, where
  *attack* is also a singular noun that *both* cannot determine).

## Done when

- Fused *each* is a Complement where nothing it could determine follows: *create one of each*
  reads as a partitive whose oblique is fused *each*, and the seven units above gain no tree.
- The fused-*each* clause of `NonNumericComplement`, `QuantityPartitiveOfEach` and
  `NominalPredicative`'s fused-*each* exclusion are deleted, or each one that stays has a comment
  naming the card and structure it still prevents.
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for the changed crates.

## Pointers

- `crates/english/grammar/english.fug`: `NonNumericComplement`, `QuantityPartitiveOfEach`,
  `NominalPredicative`, `FusedQuantifierNominalPhrase`, `FusableQuantifier`.
- `crates/syntax/src/step.rs`: `track_omitted_complement`, `track_ellipsis`.
- `docs/design/english.md` §3 (same-head complement priority), §8.
