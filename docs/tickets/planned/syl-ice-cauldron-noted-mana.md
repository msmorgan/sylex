---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, syl-tocast-noun-phrase]
---
# *Ice Cauldron*'s noted mana

## Why

*Ice Cauldron* cannot be spelled. `.syl` has no action that notes information, no amount that
reads a noted amount, and no way to say "the last card exiled with this artifact".

## Design

- `noteManaSpent` is a nullary Action that introduces no referent. It writes the information a
  linked ability reads back [CR#607.2e]: the type through the existing `ofTheLastNotedType`, the
  amount through a new reader beside it.
- "The last <p> exiled with x" is `superlative(max, zoneEntry, p)`, where `zoneEntry` is the
  timestamp of the object's entry into its current zone [CR#613.7d]; it is an object axis,
  refused for players.

## Done when

- `noteManaSpent`, the noted-amount reader and the `zoneEntry` axis exist, named against the
  game-model glossary.
- `plugins/canon/cards/Ice Cauldron.syl` spells both abilities in full, including the exile, the
  cast permission "for as long as it remains exiled", the charge counter, the note, the
  activation restriction and "Spend this mana only to cast the last card exiled with this
  artifact", whose `toCast` takes the noun phrase `syl-tocast-noun-phrase` allows. If a step does
  not resolve, the refusal is reported rather than worked around with a raw constructor.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/primitives/producedMana/ofTheLastNotedType.syl`
- `plugins/builtin/primitives/predicate/superlative.syl`
