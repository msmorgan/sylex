---
needs: [engine-payment-obligation-window]
---
# Harden the payment transaction's reachable boundaries

## Why

The clone-based payment transaction has reachable gaps that share frame reconstruction,
mana-ability classification, activation legality and runner machinery:

- Declining can fail: forced-retained records are collected (`collect_forced_retained_record_ids`
  in `crates/engine/src/payment.rs`) without the records they depend on.
- `Ability::mana_profile` (`crates/core/src/ability.rs`) skips [CR#605.1a]'s library clause, so an
  activated ability that moves a card to or from a library counts as a mana ability.
- `can_activate_mana` (`crates/engine/src/activate.rs`) passes `blanket_applies: false` to
  `legal::cant_activate`, so every blanket `Cant(Activate)` row skips mana abilities. Linvala,
  Keeper of Silence wrongly lets an opponent's creatures tap for mana. The auto-tap source scan in
  `autotap_for_cast` (`crates/engine/src/cast.rs`) hard-codes the same exemption. The doc
  comments on `cant_activate_rows` and `cant_activate`, and the comment at the auto-tap scan, cite
  [CR#702.61b] for a general blanket exemption, but that rule covers split second only.
- `legal_actions` (`crates/engine/src/legal.rs`) and the payment prompt's mana-ability list
  (`crates/engine/src/payment/mana.rs`) scan only the battlefield, so Elvish Spirit Guide's hand
  ability is never offered.
- A created delayed or reflexive trigger always fires through `TriggerFired`, never as a triggered
  mana ability (`scan_delayed` in `trigger.rs`, `scan_created_reflexive` in `resolve/effect.rs`).
- `automatic_activation_cost_usable` (`payment.rs`) checks `spendable_pool(controller, source)`:
  the source permanent rather than the announced ability is the spend subject.
- `begin_optional_payment` panics (`expect`) on a cost that is not yet concrete.
- The mana-reversal candidate loop enumerates `2^n` masks, and `checked_shl(..).unwrap_or(0)`
  silently yields no candidates at 64 actions.
- `rescindable` lists fulfillments while progress is not `Idle`, where
  `rescind_payment_fulfillment` rejects them.

## Design

- Decline reconstruction keeps the dependency closure of every forced-retained record, and
  retains the whole frame when it cannot compute that closure.
- `DeonticAction::Activate` gains an ability-kind scope (not mana; not mana or loyalty). Only a
  scoped row and split second exempt mana abilities from a blanket `Cant(Activate)`.
- An unresolved optional-payment shape (Hybrid, Phyrexian, `ManaCostOf` and any other) exposes
  its choice before the frame starts or fails plugin/runtime validation; it never panics.
- Runner affordability advice treats a source as unusable when it cannot prove a rider match.

## Done when

- `DeclinePayment` is an unconditional exit: it never returns `DecisionError` and never leaves an
  unpayable announcement open. A record retained under [CR#734.1] keeps the records it depends
  on.
- `Ability::mana_profile` applies every [CR#605.1a] criterion, including "its cost and effect
  don't move any card to or from a library", inspecting nested `Move`, `MoveGroup`, draw,
  selection and cast shapes in both cost and effect. Modal `ByAnnouncedMode` classification is
  preserved.
- A created delayed or reflexive triggered ability [CR#603.7,603.12] that meets the [CR#605.1b]
  criteria is a triggered mana ability: its by-value representation carries that, and it
  resolves through the immediate `ResolveTriggeredMana` path, never through
  `TriggerFired`/`pending_triggers` and never on the stack [CR#605.4a], keeping causal order.
- A blanket `Cant(Activate)` row blocks every matching activation, mana abilities included
  [CR#101.2,602.5]; mana abilities have no general exemption [CR#605.3]. Only split second
  [CR#702.61b] and a row scoped "unless they're mana abilities" (Damping Matrix, Pithing Needle)
  bypass it, at both `can_activate_mana` and the auto-tap scan.
- Priority actions and mana-payment prompts find sources and abilities through one shared rule
  that enumerates every zone an activated ability functions from, honoring
  `ActivatedAbility::from` [CR#113.6j,113.6m]; a hand mana ability is activatable with priority
  and inside a payment window [CR#605.3a].
- Runner advice uses the same spend subject and rider check as exact `ManaCoverage` validation
  [CR#106.6]. Advice never changes legality.
- Affordability stays out of announcement-time legality.
- Mana-reversal selection does not enumerate every subset; it preserves dependency ordering,
  [CR#734.1] barriers, independent `ManaActionId` selection and the runner's maximal-reversal
  policy, with no fixed-width limit on the number of candidate actions.
- The prompt lists a fulfillment as rescindable only while payment progress is `Idle`.
- Each item above has a focused regression, including:
  - declining after fulfillment A moves a card from battlefield to graveyard and B moves that
    reminted card from graveyard to a library (retained under [CR#734.1]) reconstructs a legal
    state and emits the incident;
  - library movement in the cost and in the effect each disqualify a mana ability, with a
    non-library control and modal `ByAnnouncedMode` unchanged;
  - separate Linvala, Keeper of Silence, Damping Matrix and Sudden Shock (split second) cases:
    Linvala blocks matching mana abilities, the other two permit them;
  - Elvish Spirit Guide is activatable from hand with priority and inside a payment window;
  - qualifying delayed and reflexive triggers stay stackless and causally ordered, including
    inside a nested payment window;
  - a restricted-mana fixture whose rider matches the source permanent but not the announced
    ability, and its inverse, and the runner never loops between activating a source and
    declining the same payment;
  - Hybrid, Phyrexian and `ManaCostOf` optional payments run without a panic;
  - reversal selection with 64 or more candidate actions;
  - a suspended fulfillment is not listed as rescindable.
- Tests and fixtures that bless library-moving mana abilities (`payment_replay.rs`'s
  `suspending_barred_mana_source`, `submitted_mana_action_with_a_library_barrier_is_forced_to_remain`)
  or blanket exemptions (`cant_activate_battlefield_blanket_grant_blocks_nonmana_allows_mana` in
  `legal.rs`) are rewritten or inverted. The split-second test in `legal.rs` stays.
- No panic or softlock on malformed or unpayable input. `crates/engine/tests/payment.rs`,
  `payment_replay.rs`, `payment_mana.rs` and `pip_payment.rs` pass.

## Not in scope

- Generalizing replay identity (`engine-payment-replay-identity-generalization`) and
  cost instructions the payment protocol cannot run (`engine-cost-instruction-gaps`).

## Pointers

- `crates/engine/src/payment.rs`, `crates/engine/src/payment/` (`replay.rs`, `mana.rs`,
  `fulfill.rs`, `coverage.rs`)
- `crates/engine/src/legal.rs` (`cant_activate`, `legal_actions`), `crates/engine/src/activate.rs`,
  `crates/engine/src/cast.rs` (`autotap_for_cast`)
- `crates/core/src/ability.rs` (`mana_profile`, `is_triggered_mana_ability`),
  `crates/core/src/deontic.rs` (`DeonticAction::Activate`)
- `crates/engine/src/trigger.rs`, `crates/engine/src/resolve/effect.rs`
