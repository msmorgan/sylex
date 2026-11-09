---
needs: [syl-argument-naming]
---

# Declarations whose name or spelling says something else

## Why

Three declarations are named or spelled for one use rather than for what they build:

- `modifiesPrecedingAction` (`primitives/timing/`) is a `Duration` spelled "It can't be
  regenerated", though it is any rider that modifies the action before it [CR#608.2c].
- `thisObject` at `LookbackSpan` (`primitives/timing/`) is spelled "spells you cast".
- `splitOverPermanent` (`templates/pronoun/`) is spelled "any target".

`syl-argument-naming` rewrites 4 arguments that hold these declarations' calls, and lands first.

## Done when

- Each is named for what it builds and spelled with words that mean it, or its one use moves into
  a Template that carries the printed wording; every caller in `plugins/` is updated.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/timing/{modifiesPrecedingAction,thisObject}.syl`,
`plugins/builtin/primitives/period/thisObject.syl`,
`plugins/builtin/templates/pronoun/splitOverPermanent.syl`.
