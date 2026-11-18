---
needs: []
---

# A noun's `Dual` relative gender serves no card

## Why

Nine nouns declare `relative_gender: Dual` (*creature*, *planeswalker*, *commander*,
*Ring-bearer*, *attacker*, *blocker*, *outlaw*, *party*, *team*), which leaves their
`RelativeGender` unvalued so that they take both *who* and *which*. The comments justify it by
CGEL's *the dog who*/*which* (Ch. 5 §17.3, pp. 497–498), and the last-conjunct rule for a
coordinated antecedent by *the books and people who* (Ch. 12 §3.5.1, p. 1049). In card text
*who* follows only *player(s)*, *opponent(s)* and two card names (*Woman Who*, *Those Who*), and
no unit prints any of the nine nouns before *who* or *which* (`units.text`, forest database).
Admission follows card text (`docs/design/english.md` §4), so the value is kept for a form no
card prints.

## Done when

- No noun declares `Dual`, or a card whose text needs it is named in its declaration; the
  Lexicon doc comment and the `feature RelativeGender` comment in
  `crates/english/grammar/english.fug` name card text, not *the dog who*.
- The coordinated-antecedent rule is kept only if a card prints a coordinated antecedent of
  *who* or *which*, which is named; otherwise it goes with the value.
- The census shows no face lost and no verdict change; units whose Readings change are reported.

## Pointers

- `crates/english/grammar/english.fug`: `feature RelativeGender`, `RelativeAntecedent`.
- `crates/vocabulary/lexicon/english/nouns.ron` (its header comment),
  `crates/vocabulary/catalogs/types.ron`, `crates/vocabulary/catalogs/designations.ron`.
