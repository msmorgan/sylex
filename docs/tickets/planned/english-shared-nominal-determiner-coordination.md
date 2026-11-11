---
needs: []
---
# Read coordinated determiners that share one nominal

## Why

*Balthor the Defiled* prints "Each player returns all black and all red creature cards from their
graveyard to the battlefield.": two determiners, each with its own premodifier, over one printed
"creature cards". The grammar has no coordination for it. `cargo xtask english parse` reads the
object only as a coordination of two whole noun phrases, "all black" (with "black" as a bare
nominal head) and "all red creature cards", so the first conjunct is misread and nothing says
"all black" ranges over creature cards. A line of the same shape with participles, "Each player
returns all tapped and all attacking creatures to their owners' hands.", has no Reading at all.

## Design

- A noun phrase may coordinate determiners with their premodifiers over one shared nominal printed
  once after the last conjunct, each conjunct's determiner determining the shared nominal with its
  own premodifiers. The coordination is the grammar's own, with no Catalog item named; if it needs
  a special case for *Balthor the Defiled* or its words, the implementer stops and reports.

## Done when

- *Balthor the Defiled*'s line has a Reading in which "all black" and "all red" each determine
  "creature cards", and that Reading's ranking key is the cheapest; the "all tapped and all
  attacking creatures" line has that Reading too, in a grammar test.
- `cargo xtask english strata --check` and `cargo test -p sylex_english` pass.

## Pointers

- `crates/english/grammar/english.fug` (`Determined`, the noun-phrase coordinations)
- `docs/decisions/english.md` §3, §4
