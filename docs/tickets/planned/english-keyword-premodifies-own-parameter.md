---
needs: []
---
# A bare keyword premodifies its own Keyword Parameter

## Why

`KeywordPremodifier` takes a bare keyword before any `BareNominal`, and a number or mana symbol
can head one (`NumeralNominal`, `SymbolNominal`). So *a 1/1 red and green Dragon creature token
with flying and devour 2* (Dragon Broodmother) has, beside the correct *[devour 2]*, a Reading
with the keyword *devour* as premodifier of the nominal *2*. The same holds for *crew 2*,
*toxic 1*, *firebending 4*, *equip {2}* (Blacksmith's Talent) and *ward {1}* (Dancing Sword),
each inside a coordination after *with*.

Two more premodifier Readings stand beside a correct one for other reasons:

- *Myriad [Pools]* inside the card name The Myriad Pools.
- *[first strike and equip] abilities* (Nahiri, Storm of Stone), beside *first strike and
  [equip abilities]*.

## Open questions

- Is a numeral or symbol Nominal ever premodified on a real card, by a keyword or anything else?
  If not, the two Productions build an unmodifiable Nominal and the keyword gate is unneeded.
- Does a card's own name segment before the grammar reads *Myriad* as a keyword?

## Done when

- None of the units above has a Reading with a keyword premodifying its own parameter, and each
  keeps its correct Reading.
- The census shows no verdict change on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `KeywordPremodifier`, `NumeralNominal`, `SymbolNominal`.
