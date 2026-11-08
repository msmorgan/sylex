---
needs: [engine-payment-obligation-window]
---

# A TUI payment overlay over the full payment protocol

## Why

The TUI never shows a payment: payment decisions are not interactive, so the driver hands
them to the auto-decider, which answers with the engine's runner shim `auto_payment_pending`
(`crates/engine/src/payment.rs`). That shim covers only the monocolor demo decks (it activates the first payable mana ability, picks deterministic
witnesses and submits). A human cannot choose which mana pays which pip, which permanents pay a
cost, or decline.

## Done when

- The TUI has a payment overlay over the engine's whole payment protocol: exact floating mana to
  pip coverage (including riders and `PayPips` alternatives), activating mana abilities in the
  middle of a payment, the payer's choice of legal cost order, whole-batch choices for action
  costs, `RescindFulfillment`, `DeclinePayment`, `SubmitPayment`, and any `ChooseManaReversals`
  decision a decline produces.
- Convenience stays in the client: forced and nullary steps and obvious payments collapse
  automatically, but the player can expand the overlay and override every legal choice before
  submitting.
- Replay barriers and retained actions are shown, with no tournament-policy penalties.
- Payment decisions are interactive in the TUI, and the human player's payments no longer go
  through `auto_payment_pending`.

## Not in scope

Hiding an unsubmitted announcement from other players over a network.

## Pointers

`crates/engine/src/payment.rs`, `crates/engine/src/payment/`,
`crates/tui/src/driver.rs`, `crates/tui/src/interact.rs`.
