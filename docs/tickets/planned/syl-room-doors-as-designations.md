---
needs: [syl-single-face-card-call, cleanup-step6-surface]
---

# Model Room doors through the unlocked designations

## Why

`.syl` declares "left half unlocked" and "right half unlocked" as designations
[CR#709.5c], but nothing reads or sets them. Room door state and its changes
[CR#709.5f..709.5j] go through separate primitives instead: a `Door` sort (`thisDoor`,
`doorOf(state: Option(LockState), room)`), `unlock(door)` and `fullyUnlock(room)`. The rules
define a locked half as one without its unlocked designation, unlocking as gaining it, and fully
unlocking as ending up with both, so two representations of one state sit side by side.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

Written against the surface (`cleanup-step6-surface`).

## Done when

- Door state and unlocking are written through the designation primitives (`gainDesignation`,
  `hasDesignation`, and a way to lose a designation for locking [CR#709.5g], which `.syl` does not
  have yet) with `leftHalfUnlocked` / `rightHalfUnlocked`, or a typed half selector over them.
  `Door`, `LockState`, `unlock` and `fullyUnlock` are deleted or become Templates over that, so
  "this door" [CR#709.5j], "unlock" [CR#709.5f] and "fully unlock" [CR#709.5i] keep their
  spellings.
- Glassworks // Shattered Yard ("When you unlock this door"), Balemurk Leech ("Whenever you fully
  unlock a Room") and Ghostly Keybearer ("unlock a locked door of up to one target Room you
  control") keep their printed wording.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/designation/leftHalfUnlocked.syl`,
`rightHalfUnlocked.syl`; `plugins/builtin/primitives/action/unlock.syl`, `thisDoor.syl`,
`doorOf.syl`, `fullyUnlock.syl`, `gainDesignation.syl`;
`plugins/builtin/primitives/predicate/hasDesignation.syl`;
`plugins/builtin/definitions/subtype/enchantment/room.syl`; the three canon cards above.
