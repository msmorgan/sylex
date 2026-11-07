---
needs: [engine-copy-spells]
---
**Engine: copies that retarget (choose new targets, a specified new target, one copy per thing
the spell could target).**

## Why

Core already spells the copy-targeting variants, but the engine executes only `CopyRetarget::AsIs`.
The `Action::CopySpell` arm in `resolve/player_action.rs` returns no work for `MayChooseNew` and
`TargetsThat`, and for a `CopySource::SelfCard` source, so those copies are silently never made.

- **Specified new target** [CR#707.10e]: Frontline Heroism ("then copy that spell. The copy
  targets that token."). Every target slot of the copy becomes that object. If the object is not
  legal for every slot, no copy is created. If a replacement effect has made several candidates
  (Anointed Procession doubling the token), the copy's controller chooses one of them as the
  target.
- **One copy per object it could target** [CR#707.10d]: Zada, Hedron Grinder and Mirrorwing
  Dragon. Core composes this as `Each(InChosenOrder(ValidTargetsFor(spell), You), CopySpell(…,
  TargetsThat(…)))`, and the engine already evaluates `Selection::ValidTargetsFor`, which
  intersects the legal sets of all slots. But `Selection::InChosenOrder` ignores its chooser and
  keeps the inner order (`resolve/query.rs`), so the controller never orders the copies on the stack.
- **May choose new targets** [CR#707.10c]: the copy keeps any unchanged targets, even illegal
  ones. Every changed target must be legal, and the copy goes on the stack with the targets that
  were decided.

## Done when

- `CopySpell` executes `MayChooseNew` and `TargetsThat` as above, and the `AsIs`-only guard in
  `resolve/player_action.rs` is gone. A `CopySource` the engine does not execute fails loudly
  instead of fizzling.
- `InChosenOrder` surfaces the chooser's ordering decision, and the
  `in_chosen_order_preserves_the_inner_group_pending_the_choice_seam` test in
  `resolve/query.rs` is rewritten to assert the chosen order.
- Tests grounded in Frontline Heroism (including two candidate tokens under a doubler), in Zada,
  Hedron Grinder (one copy per other creature, in the order the controller chose, with no copy for
  a creature that is illegal for some slot), and in Twincast (new targets, plus an unchanged
  illegal target that is kept).

## Not in scope

- "Becomes the target" events for a copy's targets: `engine-spell-copy-stack-lifecycle`.

## Pointers

- `crates/core/src/action.rs` (`CopyRetarget`, `Action::CopySpell`)
- `crates/core/src/selection.rs` (`ValidTargetsFor`, `InChosenOrder`)
- `crates/engine/src/resolve/player_action.rs` (the `CopySpell` arm)
- `crates/engine/src/step/stack.rs` (`Copied::apply`)
- `crates/engine/src/resolve/query.rs` (`ValidTargetsFor` and `InChosenOrder` evaluation)
- `plugins/canon/cards/Twincast.syl`
