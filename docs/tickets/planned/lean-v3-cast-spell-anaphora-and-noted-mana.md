---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# `toCast` over a noun phrase, and *Ice Cauldron*'s noted mana

## Why

Two gaps in `.syl`'s mana spending, separately landable:

1. `toCast(predicate: Predicate)` (`plugins/builtin/primitives/mana/toCast.syl`, "to cast
   <predicate> spells") cannot take an anaphor, so "to cast that spell" and "to cast those spells"
   cannot be written. Its three canon sites also print "spells" twice: *Ancient Ziggurat* and
   *Mishra's Workshop* pass `and([…, isSpell])` to a spelling that already ends in "spells".
2. *Ice Cauldron* cannot be spelled. `.syl` has no action that notes information, no amount that
   reads a noted amount, and no way to say "the last card exiled with this artifact".

## Design

- `toCast(spells: NounPhrase)`, so "to cast that spell" and "to cast those spells" name an
  antecedent; a generic plural ("to cast creature spells") is `bare(<predicate>)`.
- `noteManaSpent` is a nullary Action that introduces no referent. It writes the information a
  linked ability reads back [CR#607.2e]: the type through the existing `ofTheLastNotedType`, the
  amount through a new reader beside it.
- "The last <p> exiled with x" is `superlative(max, zoneEntry, p)`, where `zoneEntry` is the
  timestamp of the object's entry into its current zone [CR#613.7d]; it is an object axis,
  refused for players.

## Done when

- `toCast` takes a NounPhrase, and *Ancient Ziggurat*, *Mishra's Workshop* and *Unblinking
  Observer* write `toCast(bare(…))`, each predicate reading its printed words once (re-grep
  `toCast` in `plugins/` when starting).
- `noteManaSpent`, the noted-amount reader and the `zoneEntry` axis exist, named against the
  game-model glossary.
- `plugins/canon/cards/Ice Cauldron.syl` spells both abilities in full, including the exile, the
  cast permission "for as long as it remains exiled", the charge counter, the note and the
  activation restriction. If a step does not resolve, the refusal is reported rather than worked
  around with a raw constructor.

## Not in scope

- A cast permission publishing "that spell" to the text after it (*Rogue Class*, *Summon: Esper
  Valigarmanda*): `discourse-cast-permission-publishes-spell`.

## Pointers

- `plugins/builtin/primitives/mana/` (`toCast.syl`, `spendOnly.syl`, `ofTheLastNotedType.syl`)
- `plugins/builtin/primitives/predicate/superlative.syl`, `plugins/builtin/templates/determiner/bare.syl`
- `plugins/canon/cards/Ancient Ziggurat.syl`, `Mishra's Workshop.syl`, `Unblinking Observer.syl`
