---
needs: []
---
# Let the owner arrange cards that arrive together in a graveyard or library

## Why

When an effect or rule puts two or more cards into the same graveyard at the same time,
their owner may arrange them in any order [CR#404.3]. The engine never offers that choice. Every
arrival is appended in commit order (`Zone::Graveyard` in the zone-change placement in
`crates/engine/src/step/mod.rs`), so a destroy-all, a state-based-action batch, a multi-card
discard or a mill fixes the graveyard order by the event log's position inside the batch, which
engine law 7 forbids. Libraries are only partly covered. A `MoveGroup` with an anchor, and a
scry-family `Each` that repositions cards, surface an arrangement [CR#401.4]. Any other
simultaneous arrival at one library position is inserted in commit order by the same placement
code.

## Done when

- Two or more cards entering one graveyard in one simultaneous batch surface one arrangement
  decision to their owner (one per owner when the cards have different owners), and the
  graveyard ends in the chosen order [CR#404.3]. A single card, or a batch whose cards all go to
  different graveyards, surfaces nothing.
- Every simultaneous arrival of two or more cards at one position of one library surfaces the
  owner's arrangement [CR#401.4], not only `MoveGroup` with an anchor and the scry family, and
  the order is not revealed.
- No graveyard or library order depends on the commit order inside a batch.
- Engine tests use real cards: a destroy-all that sends two of one player's creatures to the
  graveyard, a state-based-action batch, and a mill of two or more cards.
- `cargo test -p sylex_engine` and `cargo clippy -p sylex_engine --tests -- -D warnings` pass.

## Not in scope

Melded and merged permanents [CR#712.21a,730.3a]. The engine represents
neither today. When they are added, a melded or merged permanent put into its owner's graveyard
or library must reach that same arrangement decision.

## Pointers

- `crates/engine/src/step/mod.rs`: the zone-change placement (`match to`, the
  `Zone::Graveyard` and `Zone::Library` arms), `apply_occurrence`, `arrange_piles` and
  `arrange_group_landing`.
- `crates/engine/src/resolve/action.rs`: the `Action::MoveGroup` arm, which schedules
  `WorkItem::ArrangeGroupLanding` only when it has an anchor.
- `crates/engine/src/resolve/effect.rs`: the `Instruction::Each` arm's `arrange_scope`, armed
  by `body_repositions_ordered`.
- `crates/engine/src/agenda.rs`: `WorkItem::ArrangePiles` and `WorkItem::ArrangeGroupLanding`.
- `crates/engine/src/decide/pending/choice.rs`: `ArrangePile`.
- `docs/decisions/engine.md` law 7.
