---
needs: []
---

# *Different* takes no *from* Complement

## Why

*Jason Bright, Glowing Prophet*'s "Whenever a Zombie or Mutant you control dies, if its power was
different from its base power, draw a card." cannot reach its intended Reading. The adjective
*different* declares no Complement frame, so "from its base power" attaches only as an Adjunct of
the copular Verb Phrase, never as the Complement *different* selects.

Reproduce with `cargo xtask english forestwalk --card "Jason Bright, Glowing Prophet" --line 1`:
*different* is `AdjectivePhrase [Valence:AdjectivePhrase/Bare]` in both Derivations.

## Done when

- *Jason Bright, Glowing Prophet* line 1 has a Reading in which "from its base power" is the
  Complement of *different*, and no Reading in which it is an Adjunct of the Verb Phrase.
- A judgment entry on *Jason Bright, Glowing Prophet* witnesses it.
- "with a different name than each Aura you control" (*Light-Paws, Emperor's Voice*) and "a
  different name than that spell" (*Tibalt's Trickery*) keep their Readings.
- The census before and after shows no face lost, and every unit whose verdict changes is
  checked.

## Pointers

- `crates/vocabulary/lexicon/english/adjectives.ron`: *different*, beside *equal* and its *to*
  Complement.
- `crates/english/grammar/english.fug`: `valence AdjectivePhrase`, `NonScalarIndirectComplement`.
- CGEL, Ch. 13 §5.4, pp. 1143–1145.
