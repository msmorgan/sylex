---
needs: [syl-primitive-constructors]
---

# Single-faced cards as an explicit call

## Why

A card file written `card { name = …, … }` is read as `singleFaced(…)` around a `CardFace` whose
`characteristics` wrap the record's fields. Those are two implicit wraps, and `.syl` allows no
implicit wrap but `Option` (`docs/decisions/syl.md` §2 and §3). The loader keeps them only so the
single-faced cards need no respelling.

## Design

- A card face is its Characteristics: a single-faced card is a call on the card's
  characteristics, with no `characteristics` record between them.

## Done when

- Every single-faced card in `plugins/canon/` is written as an explicit call, and every face of a
  multi-faced card is written without the `characteristics` wrap.
- The loader no longer accepts a record at `Card`, or a `CardFace` record without
  `characteristics` (the shorthand in `lean/SylGen/Resolve.lean`), each refused with a negative
  test, and `printed_name` in `crates/xtask/src/dsl.rs` no longer reads a record card.
- The respelled cards mean the same: `lake exe dumpValues compare` shows no changed card row.
- `cargo xtask lean check`, `cargo xtask dsl fmt --check`, `cargo xtask dsl differential` and
  `cargo test -p sylex_syl` pass.

## Pointers

`lean/SylGen/Resolve.lean` (the `Card` and `CardFace` record cases), `plugins/canon/cards/`,
`crates/xtask/src/dsl.rs` (`printed_name`), `docs/decisions/syl.md` §2 and §3.
