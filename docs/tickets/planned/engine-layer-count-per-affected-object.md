---
needs: []
---
# A count in a distributed static reads the object it modifies, not the static's source

## Why

A static that modifies each object of a set by a count about that object ("each other non-Aura
enchantment … has base power and base toughness each equal to its mana value", Opalescence; "Each
noncreature artifact is an artifact creature with power and toughness each equal to its mana
value", March of the Machines) needs the count evaluated once per affected object. The layer pass
evaluates every count against the effect's source instead: `apply` passes `eval_numeric_op` the
effect's `watcher`, never the object it is modifying, and `resolve_count_ref` resolves the inner
region's candidate register (`It`) through `resolve_source_relative`, which returns the source.
So Opalescence would give each enchantment Opalescence's own mana value, silently.

## Done when

- A count in an `Each(SelectAll(…), Modify(It, …))` static that names the candidate reads the
  object being modified, in every layer that evaluates counts [CR#613.4b,613.4c].
- Tests: Opalescence animates a three-mana-value and a five-mana-value enchantment as 3/3 and
  5/5; the `opalescence()` fixture in `crates/engine/tests/layers.rs` reads its mana value
  instead of setting a literal 4.

## Pointers

- `crates/engine/src/layer.rs`: `apply`, `eval_numeric_op`, `eval_count`, `eval_stat_of`,
  `resolve_count_ref`, `resolve_source_relative`.
- `crates/engine/tests/layers.rs`: `opalescence()`.
