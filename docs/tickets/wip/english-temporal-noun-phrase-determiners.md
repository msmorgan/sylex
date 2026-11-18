---
needs: []
---
# Temporal Noun Phrases that restate Noun Phrase Productions with other determiners

## Why

`TemporalNounPhrase` still builds some of its phrases with Productions of its own that parallel a
`NounPhrase` Production over a different nominal and a different set of determiners:
`SerialTemporal` and `MaximumSerialTemporal` (*three times*, *up to three times*) beside
`Determined` and `MaximumDetermination`, `DeterminedTemporalQuantity` (*a number of times*) beside
`Determined`, and `TemporalLocation` (*this turn*) beside `Determined`. They take a `BareNominal`
where the Noun Phrase Productions take a `Nominal`, and each admits determiners the other refuses.
Taking the Noun Phrase a `NounPhrase` Production builds as the frequency Adjunct, as
`ComparedFrequency` and `TemporalQuantity` do, gives a second tree wherever the Noun Phrase also
holds a postmodifier that the temporal Production leaves outside: 132 units, such as *The next
time a source of your choice would deal damage to you this turn* (*Circle of Protection: Red*)
and *four or more times this turn* (*Dragon Whelp*).

## Open questions

- Which determiners does a temporal Adjunct of location or frequency take, and are the
  differences from `Determined` (`TemporalLocation` refuses articles and numerals, `SerialTemporal`
  takes every determiner) facts of English or gates (`docs/design/english.md` §6)?

## Done when

- Each of the four Productions is either the Noun Phrase its `NounPhrase` counterpart builds,
  under a gate that leaves one tree per analysis, or is shown to differ from that counterpart in
  English, with its comment saying how.
- The census shows no verdict loss on the supported corpus, every Corpus Unit whose Readings change
  is checked, and `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `category TemporalNounPhrase`, `TemporalLocation`,
  `SerialTemporal`, `MaximumSerialTemporal`, `DeterminedTemporalQuantity`, `ComparedFrequency`,
  `TemporalQuantity`, `Determined`, `MaximumDetermination`, `TemporalPostmodifier`.
- `docs/design/english.md` §6, §8.
