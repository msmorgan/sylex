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

Two grammatical premodifier Readings stand beside the intended Reading and stay in the forest
(`docs/design/english.md` §8):

- *Myriad [Pools]* inside the card name *The Myriad Pools*.
- *[first strike and equip] abilities* (*Nahiri, Storm of Stone*), beside *first strike and
  [equip abilities]*.

## Design

- Require `head.Determination != Overt` (or `DeterminerNeed != Required` after
  `english-features-as-tags`) on `KeywordPremodifier`, `QuotedKeywordPremodifier`,
  `KeywordNominalModifier` and `premodification Participle`, so determiner-demanding
  `NumeralNominal` and `SymbolNominal` (`2`, `{1}`) are never premodified by a keyword or participle
  (covering *Avatar Roku*'s *firebending 4* as well as *devour 2* and *ward {1}*), while preserving
  adjectival premodification (*a natural 20*, *an additional {G}*).

## Done when

- None of the parameter units above (*Dragon Broodmother*, *Blacksmith's Talent*, *Dancing Sword*,
  *Avatar Roku*, etc.) has a Reading with a keyword or participle premodifying its own numeral or
  symbol parameter, and each keeps its correct Reading; *The Myriad Pools* and *Nahiri, Storm of
  Stone* keep their Readings.
- The census shows no verdict change on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `KeywordPremodifier`, `NumeralNominal`, `SymbolNominal`.
