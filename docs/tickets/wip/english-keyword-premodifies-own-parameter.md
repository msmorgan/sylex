---
needs: [english-category-restriction, english-features-as-tags]
---
# A bare keyword premodifies its own Keyword Parameter

## Why

A bare keyword is a `Premodifier` before any `BareNominal`, and a number or mana symbol
can head one (`NumeralNominal`, `SymbolNominal`). So *a 1/1 red and green Dragon creature token
with flying and devour 2* (*Dragon Broodmother*) has, beside the correct *[devour 2]*, a Reading
with the keyword *devour* as premodifier of the nominal *2*. The same holds for *crew 2*,
*toxic 1*, *firebending 4* (*Avatar Roku*, through `premodification Participle`), *equip {2}*
(*Blacksmith's Talent*) and *ward {1}* (*Dancing Sword*), each inside a coordination after *with*.

Two grammatical premodifier Readings stand beside the intended Reading and stay in the forest
(`docs/design/english.md` §8):

- *Myriad [Pools]* inside the card name *The Myriad Pools*.
- *[first strike and equip] abilities* (*Nahiri, Storm of Stone*), beside *first strike and
  [equip abilities]*.

## Design

- A `Participial` value of `PremodifierKind`, which `ParticipialPremodifier` gives its
  `Premodifier` as `KeywordPremodifier` gives `Keyword`, and two joint exclusions on the one
  `BareNominal` premodification Production (`Premodifier`):
  `!(modifier.PremodifierKind = Keyword & head.DeterminerNeed = Required)` and the same for
  `Participial` (a joint exclusion tests values its children carry, so an unvalued participle
  kind could not be excluded). Determiner-demanding `NumeralNominal` and `SymbolNominal` (`2`,
  `{1}`) are then never premodified by a keyword or participle, while adjectival premodification
  stays (*a natural 20*, *an additional {G}*).

## Done when

- None of the parameter units above (*Dragon Broodmother*, *Blacksmith's Talent*, *Dancing Sword*,
  *Avatar Roku*, etc.) has a Reading with a keyword or participle premodifying its own numeral or
  symbol parameter, and each keeps its correct Reading; *The Myriad Pools* and *Nahiri, Storm of
  Stone* keep their Readings.
- The census shows no verdict change on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax` and
  `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `category Premodifier`, `Premodifier`,
  `KeywordPremodifier`, `ParticipialPremodifier`, `premodification Participle`, `NumeralNominal`,
  `SymbolNominal`.
- `crates/syntax/src/fug.rs`, `crates/syntax/src/grammar.rs`, `crates/syntax/src/compile.rs`.
