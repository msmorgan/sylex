---
needs: []
---
# Granted *warp {2}{R}* is no clause with verb *warp*

## Why

*Artifact cards and red creature cards in your hand have warp {2}{R}.* (Tannuk, Steadfast Second)
has 3 Derivations. One reads *red* as a noun postmodified by a bare relative *creature cards in your
hand have*, and makes the keyword verb *warp* (`keyword_verb:warp`, frame `Monotransitive`) the
Predicate, with the symbol Noun Phrase *{2}{R}* as its Object. The line grants the keyword ability
*warp {2}{R}* [CR#702.185a]: *have* is the verb and *warp {2}{R}* its Object.

## Done when

- Tannuk's line has no Reading with *warp* as a verb, and a real-card `+` entry with a `!`
  assertion witnesses it.
- The census shows no verdict loss.

## Pointers

- `crates/vocabulary/catalogs/keyword_abilities.ron` (Warp's verb).
- `crates/english/grammar/english.fug`: `SymbolFusedHead`, bare relatives.
