---
needs: []
---

# Engine: a resolving spell or ability neither affects nor reads a target that has become illegal, and one that doesn't resolve because all its targets are illegal is not countered

## Why

Illegal targets are still used (reachable with Pounce, Arc Lightning and Fate Transfer).
`targets_still_legal` (`crates/engine/src/resolve/targets.rs`) judges each target but returns
one bool, though its doc says the illegal targets "are then excluded from the reads".
`live_target_slot` (`crates/engine/src/resolve/query.rs`) drops only departed objects, and the
unpruned `entry.targets` reach `resolution_region_frame` and `cast::announced_effect_items` for
spells, triggered and activated abilities (`crates/engine/src/resolve/mod.rs`). A target that
gains hexproof or protection, changes controller or stops matching its filter while staying on
the battlefield is still affected: Pounce still fights, and Arc Lightning still damages a
creature that gained protection from red. Illegal targets aren't affected by the parts of the
effect they're illegal for, and a part that needs information about one doesn't happen
[CR#608.2b].

The permanent-spell branch of `resolve/mod.rs` (an Aura attaching to `entry.targets`) never
checks target legality. A permanent spell with an illegal target, other than a bestowed Aura or
a mutating creature spell, doesn't resolve [CR#608.3b].

A fizzle is stamped as a counter (latent). The four sites in `resolve/mod.rs` that remove an
entry whose targets are all illegal give it `Cause::counter(Agency::StateBasedAction, None)`,
the same shape a real `Action::Counter` produces and the cause a "countered" view narrows by
(`crates/engine/src/event.rs`). Such a spell or ability "doesn't resolve" and is removed
[CR#608.2b]; nothing counters it [CR#701.6a], and state-based actions happen only when a player
would get priority [CR#704.3]. Multani's Presence's ruling: "A spell that doesn't resolve because
its targets are illegal isn't countered, so this ability won't trigger." No countered-event view
exists yet.

## Done when

- Target legality is judged per target, and the frame and effect items see only the legal
  targets; an illegal target reads as no object.
- A permanent spell checks its targets as it resolves and, if they are illegal, goes to its
  owner's graveyard without entering, except a bestowed Aura or mutating creature spell
  [CR#608.3b]. Pacifism (in the engine's test canon) whose target became illegal is the test.
- Targets-illegal removal carries its own cause, which is neither the "Counter" verb nor the
  state-based-action agency, at all four sites. `Action::Counter` keeps the "Counter" cause.
- Tests: Arc Lightning on two creatures, one of which gains protection from red before
  resolution, damages only the other; Pounce whose opposing target gained hexproof makes no
  fight and deals no damage; a spell whose only target became illegal goes to the graveyard and
  does not fire a "whenever a spell you control is countered" trigger.

## Pointers

- `crates/engine/src/resolve/targets.rs` (`targets_still_legal`)
- `crates/engine/src/resolve/query.rs` (`live_target_slot`)
- `crates/engine/src/resolve/mod.rs` (the spell, permanent-spell, triggered and activated
  branches)
- `crates/engine/src/event.rs` (`Cause::counter`), `crates/core/src/event.rs` (`Agency`)
- `crates/engine/src/resolve/action.rs` (`Action::Counter`)
