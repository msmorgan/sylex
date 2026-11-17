---
needs: []
---
# Restrict postposed Object modifiers to active verbs and open-postmodification Objects

## Why

Two edge cases admit ungrammatical postposed Object modifiers:

1. `postposed_modifier` in `crates/syntax/src/compile.rs` skips `gap_slot` variants but not passive
   ones (`Draft::derived`), so `Ditransitive(NP,NP)/passive+by` (which drops `IndirectObject` and
   leaves `object` at child 1) derives a postposed modifier on passive *dealt* (*Mourner's Shield*:
   `"dealt [this turn] [by a source of your choice] [that shares a color …]"`).
2. Personal pronouns and proper names carry `Postmodification = Closed` (CGEL, Ch. 5, §10.2,
   pp. 429–430; §20.3–20.4, pp. 520–521) because they take no integrated nominal postmodifiers,
   and `ComplementNounPhrase` does not project `Postmodification`. `PostposedAdjective`'s
   `$.Case = Plain` misses syncretic pronouns (*it*, *you*), and `PostposedRelative` admits pronoun
   Objects outright (*Propaganda*, *Camouflage*).

## Design

- Reject any derived frame variant (`draft.derived()`) in `postposed_modifier` so only active
  ungapped phrases derive postposed Object modifiers.
- Project `Postmodification` on `ComplementNounPhrase`, require `object.Postmodification != Closed`
  on `valence VerbPhrase`'s `postposed` entries, and remove `Case` from `PostposedModifier` and
  `$.Case = Plain` from `PostposedAdjective`.

## Done when

- `postposed_modifier` derives phrases only from active ungapped `Draft`s (`!draft.derived()`), and
  *Mourner's Shield* has no postposed-modifier Reading on passive *dealt*.
- `ComplementNounPhrase` projects `Postmodification`, `valence VerbPhrase` excludes
  `object.Postmodification = Closed` on postposed Object modifiers, and *Propaganda* and
  *Camouflage* lose their pronoun-Object postposed Readings while the census loses no face or
  intended Reading.
- `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass
  for `sylex_syntax` and `sylex_english`.

## Pointers

- `crates/syntax/src/compile.rs` (`Draft::derived`, `postposed_modifier`)
- `crates/english/grammar/english.fug` (`feature Postmodification`, `category ComplementNounPhrase`,
  `category PostposedModifier`, `PostposedAdjective`, `valence VerbPhrase`)
