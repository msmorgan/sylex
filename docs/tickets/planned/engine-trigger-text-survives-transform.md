---
needs: [engine-transform]
---
# A trigger on the stack keeps its text when its source transforms

## Why

A triggered ability on the stack "has the text of the ability that created it" [CR#603.3] and
exists independently of its source [CR#113.7a]. The engine instead stores most printed triggers
by index (`StackObject::Triggered { ability, created: None }`) and re-reads the body from the
source with `derive::abilities_of_source`, which is face-aware: a permanent with its back face up
sources its printed abilities from the back face [CR#712.6d]. Triggers that fire while the back
face is up are captured by value (`showing_back` in `trigger.rs`), but a trigger that fires
front-up and whose source then transforms before it resolves is re-read from the back face's
list: a different or shorter list, so the read panics or finds the wrong ability.

These sites index the list raw and panic on a mismatch:

- `resolve/targets.rs`, `targets_still_legal`
- `trigger.rs`, `trigger_targets` and `enter_trigger_activation`
- `cast.rs`, the `StackObject::Triggered` arm of the stack-object target getter

`resolve/mod.rs` reads with `.get()` and makes the ability vanish on a miss, which is also wrong:
the trigger should resolve with its own text, not disappear.

Delver of Secrets transforms, but no current card transforms a permanent while one of its
front-face triggers waits on the stack; day/night (`engine-day-night`) is the first that will.

## Done when

- A trigger whose source transforms between triggering and resolving resolves with the text it
  triggered with, at every site above (for example by capturing the body by value for every
  printed trigger of a double-faced source).
- No re-read site falls back silently: a body the engine cannot find is a crash that names the
  defect.
- A test transforms a double-faced permanent while its front-face trigger is on the stack, and
  the trigger's targets, legality check and effect come from the front-face text. The source is a
  real double-faced card, or a minimal repro derived from one.

## Not in scope

The replacement gather (`replace.rs`, the replacement path in `cast.rs`): it iterates the list
instead of storing an index.

## Pointers

- `crates/engine/src/derive.rs` (`abilities_of_source`, `derived_abilities_of`)
- `crates/engine/src/trigger.rs` (the `showing_back` capture), `crates/engine/src/resolve/mod.rs`,
  `crates/engine/src/resolve/targets.rs`, `crates/engine/src/cast.rs`
- `crates/engine/src/transform.rs`
