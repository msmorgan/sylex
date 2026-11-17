---
needs: []
---

# Equative Adjective Phrases are not depictive adjuncts

## Why

`DepictiveAdjective`, `DepictiveAdjunct`, `AdjectivalDepictive` and `InterleavedDepictive` place no
restriction on `Grade`, and `coordination AdjectivePhrase` leaves `Grade` unshared. On 861 Corpus
Units (*Skred*'s `"Skred deals damage to target creature equal to the number of snow permanents you
control."*, *Torrent of Fire*), an equative `AdjectivePhrase` (`equal to …`, `each equal to …`)
attaches to the `VerbPhrase` twice: once as the Object's postposed modifier (`PostposedAdjective`)
and once as a depictive predicative adjunct (`DepictiveAdjective` or `DepictiveAdjunct`). On another
66 Corpus Units (*Sphere of Annihilation*, *Light-Paws, Emperor's Voice*, *Maralen, Fae Ascendant*),
a mixed comparative-equative coordination (`less than or equal to …`, `greater than or equal to …`)
attaches as a depictive adjunct of the verb.

Predicative adjuncts are a subset of predicative complements, not the converse (CGEL, Ch. 6, §2.1,
p. 529; Ch. 4, §5.3, pp. 261–263): relational equality (`equal to …`, `same as …`) denotes no
depictive or resultative state of a subject or object predicand, and a coordination fills an adjunct
function only when each coordinate can fill it (CGEL, Ch. 15, §3.1, p. 1323, rule [1]). Zero Corpus
Units rely on `DepictiveAdjective`, `DepictiveAdjunct`, `AdjectivalDepictive` or
`InterleavedDepictive` for an equative or mixed comparative-equative `AdjectivePhrase`.

## Design

- Add `Mixed` to `feature Grade`, resolve differing coordinate grades to `Mixed` on `coordination
  AdjectivePhrase` (`resolve: [Grade -> Mixed]`, keeping `$.Grade = Comparative` only where a
  bare non-PP comparative final coordinate licenses an indirect *than* complement as in *equal or
  lesser mana value than that spell* on *Saruman of Many Colors*), so `less than or equal to …`,
  `greater than or equal to …`, `equal to or greater than …` and `equal to or less than …` carry
  `Grade = Mixed` without matching `PostposedAdjective`'s `head.Grade := Equative`.
- Exclude `Grade != Equative | Mixed` on `DepictiveAdjective`, `DepictiveAdjunct`,
  `AdjectivalDepictive` and `InterleavedDepictive`.

## Done when

- *Skred* loses its `DepictiveAdjective` Readings while keeping its `PostposedAdjective` and
  `PostpositiveAdjective` Readings, and *Sphere of Annihilation* and *Light-Paws, Emperor's Voice*
  lose their `DepictiveAdjective` Readings on `less than or equal to …` without gaining a
  `PostposedAdjective` Reading; judgments pin both.
- Ablation and before/after census reports show zero lost faces on the supported corpus, and
  `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Not in scope

- Scalar comparative depictives (*less*, *greater* without an equative coordinate).

## Pointers

- `crates/english/grammar/english.fug`: `feature Grade`, `InterleavedDepictive`,
  `AdjectivalDepictive`, `DepictiveAdjunct`, `DepictiveAdjective`, `PostposedAdjective`,
  `ComparativeIndirectComplement`, `coordination AdjectivePhrase`.
- `crates/english/judgments/postposed_adjective_phrase.txt`,
  `crates/english/judgments/comparative_depictives.txt`.
