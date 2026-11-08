---
needs: []
---
# Engine: the caster announces a division of damage or counters, with at least one for each target

## Why

A player casting a spell that divides or distributes an effect announces the division while
casting, and each target must receive at least one [CR#601.2d]. A copy keeps that division, and
changing targets can't change it [CR#115.7f]. The engine never asks for the division:

- `Instruction::Distribute` in `crates/engine/src/resolve/effect.rs` splits the total with
  `split_evenly` at resolution. When there are more recipients than the total, some get 0. An
  empty group drops the total silently.
- The `Division` decision (`crates/engine/src/decide/pending/choice.rs`) is a shell whose
  `resolve` is a `todo!` naming this ticket.
- The `Instruction::Distribute` doc in `crates/core/src/effect.rs` calls the split
  "resolution-time", which contradicts [CR#601.2d].

Electrolyze, Chandra's Pyrohelix and Aerial Volley (`dealDivided` in `plugins/canon/`) are
witnesses.

## Done when

- At announcement, the caster divides the amount among the chosen targets, at least one each.
  The target count is bounded so that a legal division exists. The division is recorded on the
  stack entry, copied with the spell, and kept when targets change.
- At resolution each target that is still legal receives its announced share, and `split_evenly`
  is gone.
- The `Distribute` doc in core states the announce-time rule.
- `arc_lightning_announces_one_to_three_targets` (`crates/engine/tests/stack.rs`) extends through
  resolution. New tests grounded in Electrolyze or Chandra's Pyrohelix cover an uneven chosen split
  and a target count above the total being refused.

## Pointers

- `crates/core/src/effect.rs` (`Instruction::Distribute`, `Distribute`)
- `crates/engine/src/resolve/effect.rs` (the `Distribute` arm, `split_evenly`)
- `crates/engine/src/decide/pending/choice.rs` (`Division`)
- `plugins/builtin/primitives/action/distribute.syl`, `plugins/builtin/templates/action/dealDivided.syl`
