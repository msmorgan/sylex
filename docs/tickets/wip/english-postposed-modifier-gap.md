---
needs: []
---
# Derive postposed Object modifiers through `valence VerbPhrase` instead of `Gap = NP`

## Why

`PostposedRelative`, `PostposedBareRelative` and `PostposedParticipialModifier` match any
`VerbPhrase` with `head.Gap = NP`, supplying an `object` child and postposed modifier after a
predicate PP adjunct (*Choose a card at random that was exiled with …*). Because `VerbPhrase` also
carries `Gap = NP` from stranded prepositions and gapped `adjunct ToInf`, those three Productions steal
non-Object gaps and build double-Object VPs — both on synthetic inputs (`"Put into your hand a card
at random that was exiled with it."`, `"Spend mana to cast a card at random that was exiled with
it."`) and on real corpus cards (*Gerrard's Hourglass Pendant*, *Second Sunrise*, *Storrev,
Devkarin Lich*). `valence VerbPhrase` already derives `PostposedAdjective` through its `postposed`
field without a `Gap`.

## Design

- Extend `valence VerbPhrase`'s `postposed` derivation in `crates/syntax/src/compile.rs` to derive
  postposed marked relatives, bare relatives (past `AdjP`-complement PPs such as *at random*) and
  passive past-participial modifiers across the verb's own `object` plus selected PP complements or
  a predicate PP adjunct, and delete `PostposedRelative`, `PostposedBareRelative` and
  `PostposedParticipialModifier`.

## Done when

- Postposed modifiers attach only to the verb's own `object` through `valence VerbPhrase`'s
  `postposed` derivation, the three `head.Gap = NP` Productions are deleted, and *Gerrard's
  Hourglass Pendant*, *Second Sunrise* and *Storrev, Devkarin Lich* lose their double-Object parses.
- *Illuminated Folio*, *Infectious Rage*, *Volatile Chimera*, *Gorex, the Tombshell* and *Omenpath
  Journey* keep their postposed Readings, the census shows no Reading lost on the supported corpus,
  and `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for `sylex_syntax` and `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `PostposedRelative`, `PostposedBareRelative`,
  `PostposedParticipialModifier`, `PostposedAdjective`, `category PostposedModifier`,
  `valence VerbPhrase` (`postposed`), `adjunct ToInf` (`adjunct.Gap = NP`).
- `crates/syntax/src/compile.rs` (`postposed_modifier`).
