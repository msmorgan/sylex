---
needs: []
---
# A bound keyword with a period reads as a keyword instruction

## Why

`KeywordInstruction` makes a Sentence of a keyword line item and a period when the item carries
a Keyword Parameter (*Equip {2}.*, *Suspend X.*), and refuses a bare keyword (*Hexproof.*),
which takes no period. A Bound Keyword counts its stem as its parameter (*forestwalk*: stem
*forest*, bound *walk*), so *Forestwalk.* and *Landwalk.* read as keyword instructions. No card
prints a bound keyword with a period.

## Open questions

- Which keyword line items does a real card print with a period, and what do they share: a
  parameter that is a separate word, a cost, or a list of named keywords?

## Done when

- `KeywordInstruction` states the property the printed forms share, and a bound keyword with a
  period has no Reading as one.
- Every unit that reads as a `KeywordInstruction` keeps its Reading, and the census shows no
  verdict change on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `KeywordInstruction`, `KeywordForm`.
- `crates/vocabulary/src/lib.rs`, where a parameter pattern values `KeywordForm`.
