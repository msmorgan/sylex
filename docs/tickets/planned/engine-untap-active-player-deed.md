---
needs: []
---
# The untap step's untap is the active player's deed

## Why

In the untap step "the active player determines which permanents they control will untap. Then
they untap them all simultaneously" [CR#502.3]. The untap has an agent, the active player, and a
decision point, which permanents untap. `DeonticAction::Untap` (`crates/core/src/deontic.rs`) has
only a patient slot, and its doc comment says untapping "is a turn-based action, not a deed some
object performs, so there is no agent slot". The untap turn-based action
(`turn_based_actions` in `crates/engine/src/step/mod.rs`) untaps every eligible permanent with no
choice.

Without the agent, the engine cannot express a player-keyed cap on the untap (Winter Orb, Static
Orb, Smoke: "players can't untap more than …") or a player declining to untap (Rubinia Soulsinger:
"You may choose not to untap …"). The `.syl` corpus already writes the untap as the `untap` deed:
`doesntUntap` and `mayDeclineUntap` forbid it with `cant(any, untap(…))`, `skipUntap` is the
one-shot next-untap-step form, and Stasis skips the step with `partSkip(…, untapStep)`.

## Done when

- `DeonticAction::Untap` has an agent slot beside `what`, and its doc comment no longer denies one.
- The untap step's turn-based action is the active player's deed, with the [CR#502.3] choice of
  which permanents untap as its decision point; an unconstrained choice untaps everything eligible,
  as today.
- A player-keyed cap and a may-decline are enforced through that decision point, tested with
  Winter Orb (or Static Orb or Smoke) and Rubinia Soulsinger.
- Existing untap tests (`Cant(Untap)`, the `skip_next_untap` rider, stun counters, phasing) pass.

## Pointers

- `crates/core/src/deontic.rs` (`DeonticAction::Untap`)
- `crates/engine/src/step/mod.rs` (`turn_based_actions`, the untap arm),
  `crates/engine/src/legal.rs` (`cant_untap_rows`, `untap_forbidden_by`)
- `plugins/builtin/templates/ability/doesntUntap.syl`, `mayDeclineUntap.syl`,
  `plugins/builtin/primitives/action/skipUntap.syl`, `plugins/canon/cards/Stasis.syl`
