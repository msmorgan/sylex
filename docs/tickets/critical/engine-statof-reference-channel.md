---
needs: [core-regions-substrate]
---

# Engine: `Count::StatOf` reports "no information" for an illegal target or a gone current-only operand instead of reading 0

## Why

`Count::StatOf` (`crates/engine/src/resolve/count.rs`) already reads a live-or-LKI reference
product: a live object reads the layered view, and a departed object with a bound snapshot reads
`last_known_stat`. What it can't say is that there is no answer. A register with no product, an
illegal target and a gone operand that is not entitled to last-known information all read `0`.
The rule is that the part of the effect needing information about an illegal target doesn't
happen [CR#608.2b], which is not the same as that part happening with 0. The test
`a_value_read_over_an_unwritten_register_degrades_to_null_not_a_panic` pins the `0` reading.

The witnesses, all Foundations cards, show both sides:

- Entitled to last-known information: Heartfire Immolator after sacrificing itself as a cost
  ("It deals damage equal to its power"); Halana and Alena, Partners and Prime Speaker Zegana
  after the source leaves; Ovika, Enigma Goliath after the triggering spell leaves the stack.
  Heroes' Bane is also here: "where X is its power" is untargeted, so a departed source's power
  comes from last-known information [CR#608.2h,113.7a], and the ability does nothing only
  because the recipient is a new object [CR#400.7].
- Not entitled: Bite Down and Felling Blow when the creature you control has become an illegal
  target [CR#608.2b].

## Done when

- A `StatOf` read over an illegal target, or over a current-only operand whose object is gone,
  is "unavailable": the part of the effect that needs that information doesn't happen
  [CR#608.2b]. It is never 0 or any other stand-in value. A read entitled to last-known
  information with a bound snapshot reads the snapshot instead.
- `a_value_read_over_an_unwritten_register_degrades_to_null_not_a_panic` is rewritten to assert
  the unavailable result.
- Tests distinguish the LKI-entitled and the current-only cases on the witness cards and check
  each card's result, not only the absence of a panic.

## Not in scope

- Capturing derived power and toughness in the snapshot: `engine-lki-robustness`.

## Pointers

- `crates/engine/src/resolve/count.rs` (`Count::StatOf`, `last_known_stat`)
- `crates/engine/src/resolve/query.rs` (`eval_reference_product`)
- `crates/engine/src/activation.rs` (`ReferenceProduct`, `Value::Unavailable`)
