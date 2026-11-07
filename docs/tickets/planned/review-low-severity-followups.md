---
needs: []
---

# Small core and engine follow-ups from a code review

**Why.** A code review left a batch of low-severity defects in `crates/core` and `crates/engine`:
each is small, but each is a value that can silently do nothing, an inconsistency, or a comment
that misleads.

**Done when.**
- `PlayerAttr::Life` and `PlayerAttr::HandSize` can no longer be the target of a `PlayerMod`:
  they are readable attributes but not continuous caps, so a
  `ModifyPlayer(You, SetTo(Life, 20))` is today a silent no-op. Split the readable attributes
  from the modifiable ones, or refuse the combination with an error that names it.
- `eval_count`'s `Count::Minus` (`crates/engine/src/layer.rs`) saturates like `Plus` and `Times`
  instead of subtracting unchecked before flooring at 0.
- `trigger_multiplier_extra` (`crates/engine/src/trigger.rs`) sees a `TriggerMultiplier` that
  arrives through a keyword or composite ability, as the rest of the trigger code does via
  `flatten_composites`, with a test.
- The `CostComponent::TapTotal` arm of the "unless" cost renderer in
  `crates/engine/src/decide/mod.rs` no longer panics with `todo!`: either the seam is closed, or it
  fails with an error naming the unsupported cost (its `todo!` cites `engine-resolve-effects`,
  which is done).
- `CostComponent`'s boxing is consistent (`ManaCostOf` and `TapTotal.count` inline while `filter`
  and `Do` are boxed), or the landing reports the enum's size showing why not.
- The convoke test comment in `crates/engine/tests/pip_payment.rs` says correctly which pip
  Grizzly Bears' tap can cover (it matches both the generic and the green row).

**Pointers.** `crates/core/src/continuous.rs` (`PlayerAttr`), `crates/engine/src/player_statics.rs`,
`crates/core/src/cost.rs`, and the files named above.
