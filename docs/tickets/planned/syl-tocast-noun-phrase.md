---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# `toCast` takes a noun phrase

## Why

`toCast(predicate: Predicate)` (`plugins/builtin/primitives/mana/toCast.syl`, "to cast
<predicate> spells") cannot take an anaphor, so "to cast that spell" and "to cast those spells"
cannot be written. Its three canon sites also print "spells" twice: *Ancient Ziggurat* and
*Mishra's Workshop* pass `and([…, isSpell])` to a spelling that already ends in "spells".

## Design

- `toCast(spells: NounPhrase)`, so "to cast that spell" and "to cast those spells" name an
  antecedent; a generic plural ("to cast creature spells") is `bare(<predicate>)`.

## Done when

- `toCast` takes a NounPhrase, and *Ancient Ziggurat*, *Mishra's Workshop* and *Unblinking
  Observer* write `toCast(bare(…))`, each predicate reading its printed words once (re-grep
  `toCast` in `plugins/` when starting).
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A cast permission publishing "that spell" to the text after it (*Rogue Class*, *Summon: Esper
  Valigarmanda*): `syl-cast-permission-spell-anaphor`.

## Pointers

- `plugins/builtin/primitives/mana/toCast.syl`, `spendOnly.syl`
- `plugins/builtin/templates/determiner/bare.syl`
- `plugins/canon/cards/Ancient Ziggurat.syl`, `Mishra's Workshop.syl`, `Unblinking Observer.syl`
