---
needs: []
---

# Spinal Parasite: negative base power and toughness, end to end

## Why

*Spinal Parasite* ({5}, Artifact Creature — Insect, -1/-1) has Sunburst and
"Remove two +1/+1 counters from this creature: Remove a counter from target permanent." It is a
compact probe of four interlocking pieces: a negative base power and toughness, counters scaled
by colors of mana spent [CR#702.44a], a cost that consumes those counters, and the toughness-0
state-based action [CR#704.5f]. Core already carries a signed base value
(`StatValue::Number(Int)`), but the engine has no Sunburst, and nothing tests the negative-base
path.

## Done when

- The engine implements Sunburst: the permanent enters with a +1/+1 counter (or, if not entering
  as a creature, a charge counter) for each color of mana spent to cast it [CR#702.44a].
- Engine tests with the real card, driven through the runner and asserting by card identity
  across zone changes:
  - cast with four colors (3/3), activated twice: no counters, -1/-1, it dies [CR#704.5f];
  - cast with five colors (4/4), activated twice: one counter, 0/0, it dies;
  - cast with five colors under a +1/+1 anthem, activated twice: one counter, 1/1, it survives,
    and a third activation cannot be paid.

## Pointers

`plugins/builtin/definitions/keyword_ability/sunburst.syl`,
`crates/core/src/cost.rs`, `crates/engine/tests/pip_payment.rs` (a counter-and-cost test).
