---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, syl-printed-read-order, cleanup-step6-surface]
---
# Enlist: an optional cost to attack and its linked trigger

## Why

Enlist means "As this creature attacks, you may tap up to one untapped creature you control that
you didn't choose to attack with and that either has haste or has been under your control
continuously since this turn began. When you do, this creature gets +X/+0 until end of turn,
where X is the tapped creature's power." [CR#702.154a]. `enlist.syl` has `body = []`, and the
cost-to-attack form has no optional variant, so the static cannot be written.

Written against the surface (`cleanup-step6-surface`): `syl-printed-read-order` reorders
`gatedBy` and `staticSpecGatedBy` to their printed order (2 files), so the offered form is
written against that order.

## Design

- The static is an offered variant of `gatedBy`: an optional cost the player may pay as the
  creature attacks [CR#508.1g,702.154b], not a prohibition unless paid, and not written with
  `asEvent`. The offered form is a typed construct, not a `Bool` parameter, as `addedCost`'s
  `CostOptionality` (`optional`, `mandatory`) is.
- "That you didn't choose to attack with" reads the declaration's chosen set, not
  `isAttacking`: the cost is chosen at [CR#508.1g], before the chosen creatures become attacking
  creatures [CR#508.1k].

## Done when

- `gatedBy` has the offered form, as a typed construct rather than a `Bool` parameter.
- `.syl` has a predicate for "a creature chosen to attack in this declaration" [CR#508.1g].
- "Has been under your control continuously since this turn began" reuses `heldSince` (as
  `controlledSinceTheirTurnBegan` and `echo.syl` do); no new lookback is invented for it.
- The linked trigger [CR#607.2h,702.154b] reads the tapped creature's power, and each enlist
  instance's trigger triggers once, only for its own instance's cost [CR#702.154d].
- `enlist.syl`'s body writes the static and its linked trigger in full.
- A Vintage-legal enlist card (one of the 12, such as *Argivian Cavalier*) is in
  `plugins/canon/cards/`.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/enlist.syl`
- `plugins/builtin/templates/static/gatedBy.syl`, `plugins/builtin/primitives/static/staticSpecGatedBy.syl`
- `plugins/builtin/primitives/condition/heldSince.syl`,
  `plugins/builtin/templates/condition/controlledSinceTheirTurnBegan.syl`,
  `plugins/builtin/definitions/keyword_ability/echo.syl`
- `plugins/builtin/templates/predicate/isAttacking.syl`
