---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, cleanup-step6-surface]
---
# A noun phrase for the active player, read by phasing's phase-in

## Why

The phasing turn-based action (`plugins/builtin/rules/conferral/phasingTurnBased.syl`) phases in
"all phased-out permanents that the active player controlled when they phased out"
[CR#703.4a,502.1]. `.syl` has no noun phrase for the active player [CR#102.1], so the row reads
the phased-out permanent's present controller during that controller's turn. Phasing changes no
control [CR#702.26d], so the two agree unless a control effect ended while the permanent was
phased out [CR#702.26f]. Then the rule phases it in during the untap step of the player who
controlled it as it phased out, and the row waits for its present controller's.

Written against the surface (`cleanup-step6-surface`).

## Done when

- `.syl` has a noun phrase for the active player [CR#102.1], named against the game-model
  glossary, and the phase-in half of the row is a lookback: this permanent phased out while the
  active player controlled it.
- The row's header comment states the reading the row now makes, with no ticket slugs.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Telling a direct phase-out from an indirect one: `syl-indirect-phase-out`.
- The engine's phasing (`engine-phasing`): the simultaneous change of every permanent and how a
  phased-out permanent is treated as though it does not exist [CR#702.26b] are the engine's.

## Pointers

- `plugins/builtin/rules/conferral/phasingTurnBased.syl`, `plugins/builtin/definitions/keyword_ability/phasing.syl`
- `docs/decisions/semantics.md` §4.8 (lookback)
