---
needs: [syl-single-face-card-call]
---

# Zone words mean what they say

## Why

The `hand` and `graveyard` Templates are spelled "your hand" and "your graveyard" but their bodies
are unscoped (`zone(hand, none)`), while `yourHand` and `yourGraveyard` have the same spelling and
a scoped body. Callers use the unscoped pair to mean an owner's zone, any player's, or yours. An
object that would go to another player's hand, graveyard or library goes to its owner's instead
[CR#400.3], which is why a destination can be unscoped; no doc comment says so, and `zone`'s
`whose` parameter does not say whether it names an owner or a controller.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Each zone Template's spelling says what its body means, and no two share a spelling with
  different bodies; every caller in `plugins/` uses the one its printed words mean.
- An unscoped destination's doc comment cites [CR#400.3], and `zone`'s doc comment says what
  `whose` names.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/zone/{hand,graveyard,yourHand,yourGraveyard}.syl`,
`plugins/builtin/primitives/zone/zone.syl`,
`plugins/builtin/definitions/keyword_action/sacrifice.syl` (its destination).
