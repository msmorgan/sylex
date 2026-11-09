---
needs: []
---

# Doc comments state why these declarations are shaped as they are

## Why

Several `.syl` declarations embody a modeling choice their doc comment doesn't state:

- `changeLife` and `setLife`: setting a life total is a gain or loss of the difference
  [CR#119.5], which is why `setLife` is a Template over `changeLife(set(…))`; `setLife` cites no
  rule.
- `tapForMana`: why tapping for mana is its own event head.
- `abilityHead`: its comment says only "Authoring primitive for `Predicate.abilityHead`".
- `pro`: what its `plurality` parameter decides.

## Done when

- Each declaration above carries a doc comment stating its modeling rationale, citing the
  governing rule where one exists ([CR#119.5] on `changeLife` and `setLife`).
- `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass; `cargo xtask cite check
  --list-noncompliant` is empty and `cargo xtask cite check` reports 0 stale, after `cargo xtask
  cite bless` for newly cited rules.

## Not in scope

The `hand`/`graveyard` Templates and `zone`'s `whose`: `syl-zone-words`.

## Pointers

`plugins/builtin/primitives/action/changeLife.syl`, `plugins/builtin/templates/action/setLife.syl`,
`plugins/builtin/primitives/action/tapForMana.syl`,
`plugins/builtin/primitives/predicate/abilityHead.syl`,
`plugins/builtin/primitives/pronoun/pro.syl`.
