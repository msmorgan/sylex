---
needs: []
---
# A `Simultaneously` member that needs a choice during resolution

## Why

The `Simultaneously` arm of `run_effect` (`crates/engine/src/resolve/effect.rs`) requires every
member's `action_items` to yield only `WorkItem::Emit`, and hits a `todo!` otherwise. Several
actions instead yield a decision before they become events: choosing a mana color, a called coin
flip, choosing new targets, and choosing a number or card name to note (`resolve/player_action.rs`).

No authored card reaches this. The `.syl` users of `simultaneously` are `fight` and Heart of
Bogardan, neither with a choice-bearing member. The nearest real case is Aura swap (Arcanum Wings),
whose exchange chooses the Aura card from hand at resolution [CR#702.65a].

`Simultaneously` evaluates every member against one snapshot before any applies, while a decision
is asynchronous (surfaced as pending and resumed by `submit_decision`). So a choice-bearing member
forces a choice: the decision resolves before the snapshot, so every member still sees one state
with the outcome known; or the batch pauses mid-way and resumes with the choice substituted, which
lets other members' state move while the decision is outstanding. Do not build this ahead of a
real card; the card that needs it should inform the answer.

## Done when

- A real card needs it, and the owner has chosen between deciding before the snapshot and pausing
  the batch.
- That card resolves through `Simultaneously` with its choice, tested end to end, and the `todo!`
  is gone.

## Pointers

- `crates/engine/src/resolve/effect.rs` (`Instruction::Simultaneously` arm),
  `crates/engine/src/resolve/player_action.rs`
- `plugins/builtin/primitives/action/simultaneously.syl`,
  `plugins/builtin/definitions/keyword_ability/auraSwap.syl`
- `docs/tickets/planned/engine-simultaneous-member-breadth.md`
