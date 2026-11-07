---
needs: []
---
# Phasing

## Why

The engine stores no phasing status. `Status::PhasedOut`/`Status::PhasedIn` reads are `todo!`
seams in `crates/engine/src/target.rs` and `crates/engine/src/trigger.rs`. Core has
`Status`, `Phasing` and `StateChange::Phased` but no action that phases a permanent out or in, so
the `.syl` `phaseOut` template, the `setStatus` primitive and the `phasingTurnBased` rule have
nothing to run as. The untap step (`turn_based_actions` in `crates/engine/src/step/mod.rs`) does
the untap but not the phasing event that comes before it [CR#502.1,703.4a].

## Done when

- Permanents carry a phased-in/phased-out status that the status predicates read. Both `todo!`
  seams are gone.
- Core can express phasing a permanent out and in, and the engine executes it as a status change,
  not a zone change: no zone-change or attach/unattach triggers fire [CR#702.26d,702.26j].
- A phased-out permanent is treated as though it does not exist, except by rules and effects
  that name phased-out permanents. It is removed from combat [CR#702.26b], and resolved continuous
  effects that modify characteristics or control leave it out of their affected set
  [CR#702.26e].
- Attached Auras, Equipment and Fortifications phase out indirectly with their host and phase in
  with it [CR#702.26g,702.26h]. An attachment that phased out directly phases in attached when it
  can [CR#702.26i].
- The untap step phases out the active player's phased-in permanents with phasing and phases in
  their phased-out permanents simultaneously, before untapping [CR#502.1,702.26a]. A skipped untap
  step skips it [CR#702.26m].
- Tests use Breezekeeper (phasing keyword), Teferi's Protection (all your permanents phase out)
  and Oubliette (phases out until it leaves, tapped as it phases in).

## Pointers

- `crates/core/src/status.rs`, `crates/core/src/event.rs` (`StateChange::Phased`),
  `crates/core/src/action.rs`
- `crates/engine/src/step/mod.rs` (`turn_based_actions`), `crates/engine/src/target.rs`,
  `crates/engine/src/trigger.rs`
- `plugins/builtin/rules/conferral/phasingTurnBased.syl`,
  `plugins/builtin/definitions/keyword_ability/phasing.syl`,
  `plugins/builtin/templates/action/phaseOut.syl`, `plugins/builtin/templates/action/phaseOutUntil.syl`
