---
needs: [lean-v3-action-clause-spine, lean-generated-canon-modules, lean-retirement]
---
**The deathtouch state-based action [CR#704.5h] is not on the creature row.** Found by
`lean-rules-tables` (2026-09-07): [CR#704.5h] reads whether a source with deathtouch has dealt a
creature damage *since the last time state-based actions were checked*. `ProjAxis.markedDamage`,
`GameEvent.stateBasedActionsChecked` and the [CR#704.5g] row already exist (the row is on
`plugins/builtin/definitions/card_type/creature.syl`, landed in
`lean-v3-card-type-conferral-rows`). The [CR#704.5h] row is written, but as a standalone scoped
`sba` rule, `plugins/builtin/rules/sba/deathtouchDestruction.syl` (scope `isCreature`), not on
the creature row as decided below. What remains is moving it there.

Decided (2026-10-13, owner; `semantics-v3.md` §§4.6, 4.8; archived
`semantics-v2.md` §7, Q83, Q90):

- `GameEvent.stateBasedActionsChecked` names the check [CR#704.3]. Under the
  `semantics-v3` spine (`docs/decisions/semantics-v3.md` §4.8), the [CR#704.5h]
  row reads `did(by(a(withKeyword(deathtouch))), dealDamage(any, this),
  .duration (.between .stateBasedActionsChecked .now))` (replacing the old
  `LookbackClause` on `damage(...)`).
- Both lethal rows are conferrals scoped to the creature card type, with "destroy" as the
  instruction; regeneration replacing it is the engine's business, as for the
  existing rows. Deathtouch's declaration stays empty (2026-09-07): its rule
  lives in the table. [CR#702.2c] (combat damage assignment) is a named residue
  for the engine.

Do: move `deathtouchDestruction` onto `plugins/builtin/definitions/card_type/creature.syl` as
a `stateBased` conferral beside the [CR#704.5g] row, keeping its citations and comment
([CR#702.2b,704.5h], [CR#702.2d]) in the declaration's header, and delete the standalone file.
Its test (Lean's `okDeathtouchDestruction`, carried to Rust by `semantics-rust-rewrite`) reads the
creature row instead, its claim unchanged. Standard constraints apply.
