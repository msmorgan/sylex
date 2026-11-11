---
needs: []
---
# CGEL citations that do not support their claim, and wrong section numbers

## Why

These citations in `english.fug` were looked up and found not to support the claim attached,
or to carry the wrong section number:

- `SumQualifiedNounPhrase` cites Ch. 5 §14.2(b), pp. 445–446 for a sum adjective (*total*)
  following a quantified Noun Phrase. The page's sentence is about *net* and *gross* ("they
  follow specific sums (*fifty dollars net*)").
- `category LinkedClause` cites Ch. 15 §2.10, pp. 1319–1320 for a Linked Conjunct opened by
  *then*. §2.10 is about *so* and *yet*; *then* is not discussed there.
- p. 413 is §9.2, cited as §9.3 in `class FloatingQuantifier` and `class FusedChoice`.
- "§9.3, pp. 408–410" covers §8.5 and §9.1, in `class PartitiveQuantifier`; `Partitive` cites
  the same string.
- "§16.1, p. 445" is §14.2, in `ComparativeIndirectComplement`.
- "§7.1, p. 357" is §4, in `MaximumDetermination`.

## Done when

- Each citation above is re-read against CGEL through the `cgel-reference` skill, and its
  comment either cites the page and section that support the claim or drops the claim.
- No grammar behavior changes.

## Not in scope

- Citations no one has checked.
- CGEL citations that a neighboring ticket rewrites with its construct: pp. 1168–1169
  (`english-tied-for-superlatives`), p. 504 (`english-synesis-each-partitive`), p. 508 for the
  head lexeme (`english-renamed-children-head-agreement`), pp. 409–410 for *outside the game*
  (`english-outside-the-game-frame`).

## Pointers

- `crates/english/grammar/english.fug`: the declarations above.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F5, F9, §3
  "The neighboring page").
