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

## Design

- A temporal location or frequency Adjunct is a Noun Phrase (CGEL Ch. 8 §6.3, p. 698; §9, p. 715),
  so `TemporalLocation` and `SerialTemporal` take the Noun Phrase the Noun Phrase Productions build
  (`MaximumSerialTemporal` is `SerialTemporal` over `MaximumDetermination`'s). Their differences
  from `Determined` are gates, each backed by a card: a Noun Phrase that ends in its head
  (relatives are `RelativeTemporalLocation`'s, a following *this turn* or *each turn* is a second
  Adjunct), no dependent outside the determiner, and for a period noun no `Dependent`, `Numerical`
  or `IndefiniteArticle` determiner (English refuses a bare *the*, p. 698; the others are gates).
- *A number of* is number-transparent (CGEL Ch. 5 §3.3, p. 350): `TransparentQuantity` takes *a*
  as well as *any*, and `DeterminedTemporalQuantity` is `TemporalQuantity` over it.

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
