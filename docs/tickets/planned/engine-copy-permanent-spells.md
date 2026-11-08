---
needs: [engine-copy-spells]
---
# Engine: a resolving copy of a permanent spell becomes a token permanent, and "cast a copy" runs

## Why

A copy of a permanent spell becomes a token permanent as it resolves [CR#707.10f,608.3f]. The
engine instead lets it vanish: `resolve/mod.rs` gives a resolving copy for which
`is_permanent_spell` holds only `AbilityResolved`, and nothing enters the battlefield. Its
comment justifies this with [CR#707.10a], but that rule covers a copy "in a zone other than the
stack", not one that is resolving. The test `resolved_permanent_copy_vanishes_without_entering_battlefield`
(`crates/engine/tests/stack.rs`) pins the vanishing. Lithoform Engine ("{4}, {T}: Copy target
permanent spell you control. (The copy becomes a token.)") and Double Major are the witnesses.

The token that results is not "created" for replacement effects or triggered abilities that refer
to creating a token [CR#111.13,608.3f], so Anointed Procession-style doublers and "whenever a
token is created" triggers must not see it.

A copy of a double-faced permanent spell is double-faced too, with the original's face up, and
resolves into a double-faced token [CR#707.10g]. `Copied::apply` (`crates/engine/src/step/stack.rs`)
mints the copy with `ObjectStore::mint`, which always starts it `Side::Front`.

`Action::CastCopy` ("cast a copy of [object]", [CR#707.12]) is grammar only: its arm in
`resolve/player_action.rs` fizzles with no events and names this ticket as its owner.

## Done when

- A resolving copy of a permanent spell enters the battlefield as a token permanent with the
  spell's characteristics. It no longer counts as a copy of a spell, and token-creation
  replacements and "create a token" triggers do not apply to it.
- A spell copy takes the original's `side`, and a back-face-up copy resolves into a token with its
  back face up.
- `Action::CastCopy` runs the [CR#601.2a..601.2h] casting steps for a copy created in the
  object's own zone, and the cast copy is a spell on the stack [CR#707.12]. Players choose
  individually whether to cast each of several copies [CR#707.12a].
- `resolved_permanent_copy_vanishes_without_entering_battlefield` is rewritten to assert that the
  token enters. New tests are grounded in Lithoform Engine or Double Major: one shows that the
  token enters and that a token-creation replacement does not apply to it, and one covers a cast copy.
- The comments in `resolve/mod.rs` and `resolve/player_action.rs` that name this ticket as a
  follow-up are gone.

## Not in scope

- How a non-permanent spell copy leaves the stack (to the graveyard, then it ceases to exist):
  `engine-spell-copy-stack-lifecycle`.

## Pointers

- `crates/engine/src/resolve/mod.rs` (the `entry.copy && self.is_permanent_spell(spell)` arm)
- `crates/engine/src/step/stack.rs` (`Copied::apply`)
- `crates/engine/src/resolve/player_action.rs` (`Action::CastCopy`)
- `crates/engine/tests/stack.rs` (`bears_copy_game`, the pinned test)
