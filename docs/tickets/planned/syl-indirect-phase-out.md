---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# Record whether a permanent phased out directly or indirectly

## Why

An Aura, Equipment or Fortification that phases out with the permanent it is attached to phases
out "indirectly" and phases in only with that permanent [CR#702.26g]; one that phased out
directly phases in by itself [CR#702.26i]; if both would happen at once, it phases out indirectly
[CR#702.26h]. The phasing turn-based action (`plugins/builtin/rules/conferral/phasingTurnBased.syl`,
[CR#703.4a,502.1]) infers "indirectly" from the attachment: it skips any phased-out permanent
attached to a phased-out one. An attachment that phased out directly while its host stayed
phased in, and whose host phased out later, is skipped too, though the rule phases it in.

The phase-out itself is one `setStatus(phasedOut, …)` (`templates/action/phaseOut.syl`), and an
indirect phase-out records nothing of its own, so no lookback can tell the two apart today.

## Done when

- How a permanent phased out is recorded, and the row reads it: either the indirect phase-out is
  its own event (the attachment's phase-out caused by its host's), so the row reads "did not phase
  out because its host did", or the status carries the distinction. The choice follows the CR's
  structure (`docs/decisions/semantics.md` §1 item 5).
- The row's header comment states the reading the row now makes, with no ticket slugs.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The active player's noun phrase in the phase-in half: `syl-active-player-noun-phrase`.
- The engine's phasing (`engine-phasing`): the simultaneous change of every permanent and how a
  phased-out permanent is treated as though it does not exist [CR#702.26b] are the engine's.

## Pointers

- `plugins/builtin/rules/conferral/phasingTurnBased.syl`, `plugins/builtin/definitions/keyword_ability/phasing.syl`
- `plugins/builtin/templates/action/phaseOut.syl`, `phaseIn.syl`, `phaseOutUntil.syl`
- `docs/decisions/semantics.md` §1 item 5, §4.8 (lookback)
