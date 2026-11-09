---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---

# A later spell ability that acts "before" an earlier one's action

## Why

*Savage Punch*: "Target creature you control fights target creature you don't control. /
Ferocious — The creature you control gets +2/+2 until end of turn before it fights if you control
a creature with power 4 or greater." The second paragraph is its own spell ability [CR#113.2c],
yet its effect happens before the first paragraph's fight: instructions are followed in the order
written, but later text may modify the meaning of earlier text [CR#608.2c]. Nothing in `.syl` lets
one spell ability place its action before an action of an earlier one. Spelling both paragraphs as
one sequence with the pump first erases the ability boundary; spelling them in written order puts
the pump after the fight, which changes the outcome.

## Done when

- A primitive `beforeEvent(action: Action, event: GameEvent): Action` exists, spelled
  "<action> before <event>". It is read like `holdUntil`: `action` first, then `event` in the
  state `action` leaves, where `event` is the earlier ability's action that `action` precedes.
- *Savage Punch* is in `plugins/canon/cards/` with its two paragraphs as two spell abilities,
  the second written as `beforeEvent` over the pump and "it fights" inside its ferocious
  condition.
- That its "the creature you control" and "it" read the first paragraph's target (spell
  abilities share targets) is not checkable until reference resolution exists.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

A later ability replacing an earlier one's action
(`syl-cross-ability-self-replacement`), which needs a different design.

## Pointers

`plugins/builtin/primitives/action/holdUntil.syl`,
`plugins/builtin/templates/action/doOnlyIf.syl`,
`plugins/builtin/primitives/ability/{italicHead,spell}.syl`.
