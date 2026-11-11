---
needs: []
---
# A determined slash measure is marked as a compound's first base

## Why

`MeasureNominal` builds a slash measure as a noun under a determiner (*a 3/3*, *an additional
+2/+2*) and sets `$.CompoundPart = Base`. `feature CompoundPart` describes a Conjunct of a
Word-Part Coordination: a Compound Noun's first base waiting for the shared head word of the
last Conjunct (*a +2/+0, +1/+1, or +0/+2 counter*). A determined size standing alone is a
complete nominal, not a first base. The value is there because both `coordination Nominal`
schemas require `first.CompoundPart != Base`, which keeps a measure from being a non-final
Conjunct of a Nominal Coordination and so leaves the counter example one tree. The same
equation appears to refuse every Nominal Coordination whose non-final conjunct is a size (*a
2/2 or 3/3*, *a white 2/2 and blue 3/3*), since `Premodifier` passes the value up from its head.

## Open questions

- Keep the mark and say in the comment that it is a tag to block one coordination, or leave
  `CompoundPart` unvalued on `MeasureNominal` and stop `coordination Nominal` from pairing a
  bare measure with a `SharedHeadCompound` some other way?

## Done when

- Whether *a 2/2 or 3/3* and *a white 2/2 and blue 3/3* have a Reading today is confirmed by
  parse, and whether any card in the supported corpus coordinates sizes that way is recorded.
  If both parse, the ticket closes with no change.
- `MeasureNominal` carries `CompoundPart = Base` only if a size really is a first base there, or
  its comment says what the value is for.
- *A +2/+0, +1/+1, or +0/+2 counter* keeps one tree, and the census shows no Reading lost on
  the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `MeasureNominal`, `feature CompoundPart`,
  `coordination Nominal` (both schemas), `WordPartCoordination`, `MeasureBase`, `Premodifier`.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (A4).
