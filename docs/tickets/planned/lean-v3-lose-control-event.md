---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# "When you lose control of <object>" has no event

Standard constraints apply.

## Problem

`GameEvent` names a control change only from the gaining side (`changeControl`, "gains control
of … from you"). Nothing names a player's losing control of a permanent, which is not the same
event: control returns to the owner when a "gain control" effect ends, and other effects can take
it. *Stolen Uniform*'s last sentence, a delayed triggered ability its resolution creates
[CR#603.7a], needs it: "When you lose control of that Equipment this turn, if it's attached to a
creature you control, unattach it."

Other Vintage-legal cards in `data/derived/cards.jsonl` that print "lose control of": *Duplicity*,
*Khârn the Betrayer*, *Krovikan Vampire*, *Ogre Geargrabber*, *Ray of Command*, *Seraph*.

## Ruling (owner, 2026-10-24)

Approved.

## Change

Either extend `Action.changeControl` (AST in Rust `semantics`, currently the gaining side only) with
a direction, or add `Action.loseControl (subject : NounPhrase)` with primitive `loseControl`. The
owner leaves the choice between the two to the implementer; record the choice and why in the
landing. Admit it in event and lookback patterns alongside `gainControlOf`
(`plugins_v3/builtin/primitives/action/gainControlOf.syl`).

## Tests

- Extend `stolenUniformOpening` (a fragment in `lean/Semantics/Proofs/CardsChoice.lean`, carried to
  a Rust test by `semantics-rust-rewrite`) to the whole card, as
  `plugins_v3/canon/cards/Stolen Uniform.syl`.
