---
needs: []
---
# RelativeTemporalLocation restates Determined

## Why

`RelativeTemporalLocation` (*the next time you cast a spell*, *any time you could cast an
instant*, *each time you surveil*) builds a temporal Noun Phrase from `determiner:DeterminerPhrase
Nominal` with its own determiner agreement, restating `Determined` and `FreeChoiceDetermined` over
a Nominal that a relative clause bounds (`RelativeBound`). `TemporalLocation` and
`SerialTemporal` take the Noun Phrase the Noun Phrase Productions build; this Production is the
last temporal one that restates them (`docs/design/english.md` §6). `RelativeBound` is a local
feature of `Nominal`, which no Noun Phrase carries, so a bridge over `NounPhrase` cannot read it
yet.

## Done when

- `RelativeTemporalLocation` takes the Noun Phrase a Noun Phrase Production builds, under a gate
  that leaves one tree per analysis, and builds the phrases it builds today (115 units, such as
  *Circle of Protection: Red* and *Edgar, King of Figaro*).
- The census shows no verdict loss on the supported corpus, every Corpus Unit whose Readings change
  is checked, and `cargo xtask english strata --check`, `cargo test -p sylex_english`,
  `cargo clippy -p sylex_english --tests -- -D warnings` and `cargo test -p xtask` pass.

## Pointers

- `crates/english/grammar/english.fug`: `RelativeTemporalLocation`, `TemporalLocation`,
  `SerialTemporal`, `Determined`, `FreeChoiceDetermined`, `feature RelativeBound`,
  `NominalPostmodifier`, `TemporalPostmodifier`.
- `docs/design/english.md` §6, §8.
