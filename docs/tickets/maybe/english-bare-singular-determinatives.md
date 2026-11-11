---
needs: []
---
# Modifiers made determinatives because a bare singular noun follows them

## Why

Oracle prints a singular count noun with no determiner after certain words: *target creature*,
*attacking creature*, *defending player*. The lexicon handles each by entering the word as a
determinative (`closed_class.ron` has *attacking* and *defending* as singular count
determinatives beside *target*), and the grammar then patches what follows from that: the
Targeting Marker needs `ModifiedTargetingMarker`, `MarkedDeterminer`, `MarkedMaximum` and the
`TargetedQuantity` Productions to get a plural or to follow another determiner. The register
fact is one fact (these words license a bare singular count nominal), stated once per word.

## Open questions

- Is there one account of a bare singular count nominal after these words (for example a
  Production that licenses the missing determiner for a class of premodifiers) that would let
  them be the participles and marker they are, or does each stay a determinative?

## Done when

- Every lexeme entered as a determinative for this reason is listed from the lexicon, with the
  grammar declarations that exist because of it.
- The words follow the owner's ruling; if they stay determinatives, the lexicon comment says
  that this is why.
- *Target creature*, *attacking creature* and *defending player* keep a correct Reading, and
  the census shows no Reading lost on the supported corpus.

## Not in scope

- The Targeting Marker's Number under *any number of*: `english-targeting-marker-number`.

## Pointers

- `crates/vocabulary/lexicon/english/closed_class.ron` (*target*, *attacking*, *defending*).
- `crates/english/grammar/english.fug`: `category TargetingMarker`, `BareTargetingMarker`,
  `ModifiedTargetingMarker`, `MarkedDeterminer`, `MarkedMaximum`, `Determined`.
- `docs/contexts/oracle-english/CONTEXT.md` (Targeting Marker).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F8, §3).
