---
needs: [lean-v3-reciprocal-each-other, plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# "Two target creatures … fight each other": a counted pair

Standard constraints apply.

*Rivals' Duel*: "Choose two target creatures that share no creature types. Those creatures fight
each other." The checker (Lean's `DRS.distributivePair?`, `lean/Semantics/Check/Words.lean`,
ported to `discourse`) finds a pair only in a coordination's group, whose members are one
referent per mention. `targets (exactly 2) isCreature` steps to one plural referent with no
recorded count (`Payload.size` is `none`), so
`sequentially [choose (targets (exactly 2) isCreature), fightEachOther (those creature)]` is
refused `theRestFits .object`.

## Ruling (owner, 2026-10-24)

Approved.

## Change (in `discourse`)

- Record `size := some 2`, or the exact count `n` when the `Quantity` is `range (some n) (some n)`,
  on the plural referent introduced by `DetPhrase.target` / `DetPhrase.count`.
- That way `targets (exactly 2) isCreature` is recognized by the distributive-pair lookup and the
  "the other" check (Lean's `distributivePair?`, `theOtherOk`) for `fightEachOther`
  (`plugins/builtin/definitions/keyword_action/fightEachOther.syl`).
- Then spell *Rivals' Duel* as `plugins/canon/cards/Rivals' Duel.syl` (its "share no creature
  types" is `agree (.subtype .creature) .differ …`). Two targets of one "target" word are
  distinct objects [CR#115.3], so no self-fight [CR#701.14c] arises here.
- Check what the count does to other readers of `Payload.size` (Lean's `countedGroupSize`,
  `theOtherOk`).

## Tests (`discourse`)

- *Rivals' Duel* spelled in full, checking clean.
- `targets (exactly 2) isCreature` followed by `fightEachOther (those creature)` is accepted, and a
  count other than two (`exactly 3`) is still refused for the pair.
