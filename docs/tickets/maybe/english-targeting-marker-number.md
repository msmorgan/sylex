---
needs: []
---
# The Targeting Marker alone under *any number of*

## Why

The lexicon gives the Targeting Marker *target* `number: Singular`, so an outer determiner or
an adjective supplies a plural (*two target creatures*, *other target creatures*). For *any
number of target creatures*, where nothing outside the Marker does, three declarations build the
oblique: `TargetedQuantity`, `TargetedOblique` and `TargetedNounPhrase`, the last with
`determiner.Number := Singular` beside `head.Number = Plural`. `TargetedQuantity`'s comment
grounds this in number transparency (the quantity, not the Marker, counts the targets).

Two readings of this exist. In one, number transparency (CGEL Ch. 5 §3.3, pp. 349–351) gives
the whole Noun Phrase the oblique's Number and says nothing about waiving agreement inside the
oblique, so the three declarations are a bypass of the Marker's singular, which itself refuses
only *target creatures*, a form no card prints. In the other, the Targeting Marker is Oracle's
one non-English functional marker, and these declarations cleanly capture its bare-singular
default and its behavior under outer determiners and *any number of*.

`TargetedNounPhrase` also takes any singular `Targeting` DeterminerPhrase as the Marker alone,
so a determined Marker reads there: *Exile any number of another target creatures.* and *Exile
any number of a target creatures.* each have a Reading.

## Open questions

- Make *target* number-neutral and delete the three declarations, or keep them with a comment
  that says the Marker is singular in the lexicon and this is the exception for *number of*?

## Done when

- The singular on *target* is ablated and the spurious Readings and unit growth on the supported
  corpus are recorded.
- The Targeting Marker follows the owner's ruling, and no comment cites number transparency for
  more than the whole Noun Phrase's Number.
- Under *any number of*, only the Marker word, alone or after an adjective, determines the
  oblique: *Exile any number of another target creatures.* has no Reading.
- Every *any number of target* unit keeps a correct Reading, and the census shows no Reading
  lost on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `TargetedQuantity`, `TargetedOblique`,
  `TargetedNounPhrase`, `category TargetedOblique`, `category TargetedNounPhrase`,
  `category TargetingMarker`, `BareTargetingMarker`, `ModifiedTargetingMarker`,
  `MarkedDeterminer`, `MarkedMaximum`, `feature DeterminativeDistribution` (`Targeting`).
- `crates/vocabulary/lexicon/english/closed_class.ron` (*target*).
- `docs/contexts/oracle-english/CONTEXT.md` (Targeting Marker).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F8),
  `docs/evidence/english-fug-hax-check/gemini.md` (§2, Cleared).
