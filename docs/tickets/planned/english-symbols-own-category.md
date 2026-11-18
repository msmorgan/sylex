---
needs: []
---

# Symbols are their own Category, not nominals

## Why

A symbol phrase (`{2}`, `{G}{G}`, `{T}`) stands as a Noun Phrase (`SymbolNominal`,
`FusedNounPhrase -> Symbols`), so any verb that takes a Noun Phrase Object takes a symbol:
*Equip {0}. Activate only once each turn.* (*Leather Armor*) read as an imperative of the verb
*equip* with `{0}` its Object, on 15 units. That was stopped by declaring the keyword verb
*equip* Passive-only, which refuses active *equip a creature*, good English that reminder text
and the CR print. The cause is the symbol's Category, not the verb.

## Open questions

- **Where symbols stand as Noun Phrases today.** List, from the forest database, every position
  a symbol phrase fills through a Noun Phrase or nominal Production: by governing lexeme and
  frame (*pay*, *add*, *cost*, *for* …), with unit counts, and every Production that builds a
  nominal over a symbol (`SymbolNominal`, `SymbolPremodifier`, `MultipliedSymbols`, measures).
- **What selects a symbol if it is no nominal.** Propose the design: a `Symbols` Slot Kind that
  the frames of those lexemes name, which positions keep a nominal reading (if any card needs
  one), and what coordination of a symbol with a Noun Phrase (*{2} or a creature*) becomes.
  Report what each choice does to Derivation counts on a sample before building it.

## Done when

- The open questions are answered with the forest evidence, and the design is recorded here.
- A symbol phrase is no Noun Phrase: the frames that take one name it, and a verb whose frame
  takes a Noun Phrase takes no symbol.
- The keyword verb *equip* has its active frame again, and the 15 units keep the single Reading
  they have now (*Leather Armor*, *Dáin Ironfoot*).
- The census shows which Readings changed and why, with no verdict loss, and
  `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for every changed crate.

## Pointers

- `crates/english/grammar/english.fug`: `Symbols`, `SymbolNominal`, `SymbolPremodifier`,
  `MultipliedSymbols`, `NumeralNominal`.
- `crates/vocabulary/catalogs/verbs.ron` (`equip`), `crates/syntax/src/frames.rs` (`SlotKind`).
