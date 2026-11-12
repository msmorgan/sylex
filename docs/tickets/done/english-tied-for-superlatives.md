---
needs: []
---
# The *tied for* superlatives cite a page that makes the article obligatory

## Why

Three Productions build an article-less superlative as `BareNominalComplement`:
`SuperlativeBareNominal` (*tied for lowest life total*), `SuperlativeQuantifierFusedHead` over
`class CountSuperlativeQuantifier = ["det:fewest"]` (*tied for fewest*, Goblin Game) and
`SuperlativeFusedHead` (*tied for most common*, Sulam Djinn, Zanam Djinn).
`SuperlativeBareNominal` cites CGEL Ch. 13 §6.3.4, pp. 1168–1169 for the article going
unexpressed. That passage drops *the* with plural or non-count heads, says a count singular noun
"requires a determiner by the general rules of NP structure", and says *the* is obligatory with
a fused determiner-head; yet `SuperlativeBareNominal` requires exactly a singular count head.
The other two cite nothing on the article. `CountSuperlativeQuantifier` leaves out *most* and
*least* because fused they would leave *common* a depictive Adjunct (*tied for [most]
[common]*), which makes the one-member class a guard against a second tree; that also leaves
*tied for most* and *tied for least* unparsed. *For*'s frame that selects `BareNominalComplement`
also serves bare nominals such as *until end of turn*.

## Design

- `SuperlativeBareNominal`, `SuperlativeQuantifierFusedHead` and `SuperlativeFusedHead` stay,
  and so does the `slot Nominal` they share with bare nominals: the forms the shared slot
  additionally admits (*until fewest*, *tied for end of turn*) are printed on no card.
- *Most* and *least* are not fused after *tied for*: fused, they leave *common* a depictive
  Adjunct (*tied for [most] [common]*, Sulam Djinn).

## Done when

- CGEL pp. 1168–1169 is re-read and what it says about a count singular head and about a fused
  superlative determiner-head is recorded; if it supports the Productions, the ticket closes
  with only the comments corrected.
- No comment cites a page for a claim the page does not make, and `CountSuperlativeQuantifier`'s
  comment states the second tree it prevents, or the class is gone.
- Goblin Game, Sulam Djinn and Zanam Djinn keep a correct Reading, and the census shows no
  verdict change on the supported corpus; Desecrator Hag's second tree of one analysis goes.

## Not in scope

- The `=` on `SuperlativeBareNominal`'s `modifier.Grade`: `english-require-on-unvalued-features`.

## Pointers

- `crates/english/grammar/english.fug`: `SuperlativeBareNominal`,
  `SuperlativeQuantifierFusedHead`, `SuperlativeFusedHead`, `class CountSuperlativeQuantifier`,
  `class SuperlativeQuantifier`, `category BareNominalComplement`, `slot Nominal`,
  `BarePrepositionalNominal`.
- `crates/vocabulary/lexicon/english/prepositions.ron` (*for*).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F2),
  `docs/evidence/english-fug-hax-check/gemini.md` (C2).
