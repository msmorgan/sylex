---
needs: []
---
# `SingularOrQuantified` claims a quantified plural that nothing builds

## Why

`DeterminativeDistribution` has a value `SingularOrQuantified` whose only holder is *another*.
Its comment says *another* "also takes a quantified plural (*another three bodies*; CGEL
5§3.4:353)". No equation in `english.fug` names the value. A quantified plural under a singular
determinative is `DeterminedModifiedQuantity`, which needs an adjective between them (*an
additional two creatures*), and `DeterminedQuantity` requires the determiner's Number to equal
the quantity's, so *another three bodies* appears to have no Reading. The example is CGEL's, and
no card in `data/scryfall/oracle-cards.jsonl` prints *another* before a numeral. The value's one
effect is to be something other than `IndefiniteArticle`, which `TemporalLocation` excludes
(*another day*, never *a day*).

## Open questions

- Is the value a temporal-location distinction between *a* and *another* and nothing more (then
  its name and comment should say that), or should *another* over a bare quantified plural be
  built?

## Done when

- Whether *another three creatures* has a Reading is confirmed by parse, and the absence of the
  form from the supported corpus is confirmed.
- The value's name and comment describe what the grammar does with it, and every claim in the
  comment has a Production behind it.
- *Another day* keeps its Reading where `TemporalLocation` admits it, and the census shows no
  Reading lost on the supported corpus.

## Not in scope

- `DeterminedModifiedQuantity`'s missing singular requirement:
  `english-require-on-unvalued-features`.

## Pointers

- `crates/english/grammar/english.fug`: `feature DeterminativeDistribution`,
  `feature ObliqueDistribution`, `TemporalLocation`, `DeterminedQuantity`,
  `DeterminedModifiedQuantity`.
- `crates/vocabulary/lexicon/english/closed_class.ron` (*another*).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (B3).
