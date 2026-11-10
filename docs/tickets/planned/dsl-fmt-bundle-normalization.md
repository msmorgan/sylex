---
needs: [cleanup-step6-syl]
---

# `cargo xtask dsl fmt` collapses a wrap a coercion supplies

## Why

`.syl` declares three coercions (`docs/decisions/syl.md` §2): a card type and a subtype stand at
`NounWord` through `type` and `subtype`, and an integer at `Amount` through `lit`. The bare form
is the corpus's spelling, and the wrapped primitive stays callable, so `lit(3)` and `3`, and
`type(creature)` and `creature`, are both valid and resolve to one term. Nothing normalizes
them, so the corpus can drift back to two spellings of one term. Recovery already yields the
bare form (`docs/decisions/spelling.md` §4); the written corpus should match it.

The `.syl` formatter (`sylex_syl`'s printer, which `cargo xtask dsl fmt` runs) cannot do it: it
reads one file's syntax and resolves no sort, and `subtype(goblin)` is the `NounWord` primitive
at a head noun but the `TextWord` one in a text change, with identical syntax. `cargo xtask dsl
check` cannot refuse the wrapped form without making the primitive uncallable at its to-sort.
This follows step 6 (`cleanup-step6-syl`).

## Design

- `cargo xtask dsl fmt` resolves the whole bundle with the xtask resolver and rewrites each
  explicit call whose wrap a declared coercion would supply at that position (today `type(…)`,
  `subtype(…)` or `lit(…)`) to the bare form, deciding by the resolved call, never by text.
- `cargo xtask dsl fmt --check` refuses a file that holds such a wrapped call, naming the call
  and the coercion that supplies it.

## Done when

- After `cargo xtask dsl fmt`, no call in `plugins/` writes a wrap a coercion supplies, and each
  rewritten file resolves to the same tree as before.
- `cargo xtask dsl fmt --check` fails on a fixture that writes `lit(3)` at an `Amount` slot or
  `type(creature)` at a `NounWord` slot, and passes on `subtype(goblin)` at a `TextWord` slot.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Pointers

`crates/xtask/src/dsl_fmt.rs`, `crates/xtask/src/dsl_check.rs`,
`crates/xtask/src/dsl_check/coercion.rs`, `docs/decisions/syl.md` §2–§3,
`docs/decisions/spelling.md` §4.
