---
needs: []
---

# Core: read a group an enclosed per-player region produced ("… this way")

## Why

Deadly Brew: "Each player sacrifices a creature or planeswalker of their choice. If you
sacrificed a permanent this way, you may return another permanent card from your graveyard to
your hand." `.syl` writes this kind of readback as a lookback over the enclosing action
(`did(…, thisWay)`, as Vaevictis Asmadi, the Dire does), but core has no shape for it. The first
sentence's choice is made inside the `Each`-over-players body, a region of its own, and a region
reads nothing it did not declare, with only captures going in and linked memory crossing out
(`core.md` laws 1–3). Linked memory [CR#607.1] is the wrong channel: it links two abilities printed
on one object, not two clauses of one resolution, and the group wanted here is per player.
Rewriting the card as "you sacrifice, then each opponent sacrifices" is not a substitute: the
choices are made in APNAP order and the sacrifices happen simultaneously [CR#608.2e].

## Done when

- Core can represent a group produced across the iterations of an enclosed per-player region so a
  later clause of the same resolution can test membership in it and exclude its members
  ("another"), consistent with `core.md`'s region laws (or with an amendment to them agreed with
  the owner).
- `crates/engine/tests/region_witnesses.rs`'s
  `deadly_brew_lets_the_caster_rebuy_after_the_table_sacrifices` loses its `#[ignore]` and
  passes, without new syntax in `v1/`: each player sacrifices one of their own, and the caster,
  having sacrificed one this way, returns a different permanent card from their graveyard to
  their hand.

## Pointers

`docs/decisions/core.md` (Laws, §1–§3), `crates/core/src/effect.rs` (`Each`),
`crates/engine/tests/region_witnesses.rs` (Deadly Brew),
`plugins/canon/cards/Vaevictis Asmadi, the Dire.syl`,
`plugins/builtin/primitives/timing/thisWay.syl`.
