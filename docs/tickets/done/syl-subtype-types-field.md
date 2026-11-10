---
needs: [syl-subtype-card-types]
---

# A subtype writes its card types out whole

## Why

A `subtype` declaration's `kind = creature` and `kind = spell` resolved through two Templates at
`List(CardType)`, `creature` (`[creature, kindred]`) and `spell` (`[instant, sorcery]`). Every
`List(CardType)` position could reach them, so `types = creature` in `Characteristics.types`,
`TypeLineChanges.types` or `isAlso`'s `types` silently meant a kindred creature.

## Design

- The `subtype` kind's field is `types`, a `List(CardType)` listing every card type the subtype
  is correlated with [CR#205.3c]: `types = [creature, kindred]` for a creature type
  [CR#205.3m], `types = [instant, sorcery]` for a spell type [CR#205.3k], `types = [battle]`
  for a battle type. Every family is written the same way.
- No word stands at `List(CardType)`, so a lone card-type word there is refused. A list literal
  of card types adds no dependency edge (a catalog reference is no edge).

## Done when

- Every `subtype` declaration writes `types` as a list literal, and the `subtype` kind in
  `crates/xtask/src/dsl_check/schema.rs` declares `types`, not `kind`.
- No Template is declared at `List(CardType)`.
- `cargo xtask dsl check` refuses `types = creature` on a card's characteristics beside the real
  builtin bundle, with a negative test.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check/schema.rs`, `plugins/builtin/definitions/subtype/`,
`crates/xtask/tests/suite/dsl_check.rs`.
