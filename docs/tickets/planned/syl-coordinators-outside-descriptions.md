---
needs: [syl-union-membership]
specs: [kind-sorts-and-position-unions]
---

# The coordinators at the other sorts are declared once too

## Why

`syl-union-membership` declares `or`, `and`, `andOr`, `both` and `coord` once for the describing
sorts, `Nominal`, `NounPhrase` and `Test`. They remain separate declarations at `Cost`,
`Condition`, `Timing`, `Location`, `GameEvent`, `TriggerCondition`, `ProducedMana` and
`Predicate`, each with its own copy of the spelling, and `either` at `Cost` spells "$left or
$right" beside `Cost`'s `or`.

## Done when

- Each coordinator is one declaration whose `at` list names every sort it coordinates, or the
  landing says why a sort's coordinator differs (`TriggerCondition`'s `and` means what its `or` does
  [CR#603.1b,603.2]) and keeps it apart.
- No two declarations at one sort spell one coordination.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §4;
`plugins/builtin/{primitives,templates}/{cost,condition,timing,zone,event,producedMana,predicate}/`.
