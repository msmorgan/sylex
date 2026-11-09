---
needs: [syl-primitive-constructors]
---

# Single-faced cards as an explicit call

## Why

A card file written `card { name = …, … }` is read as `singleFaced(…)` around a `CardFace` whose
`characteristics` wrap the record's fields. Those are two implicit wraps, and `.syl` allows no
implicit wrap but `Option` (`docs/decisions/syl.md` §2 and §3). `cargo xtask dsl check` accepts
them only so the single-faced cards need no respelling.

## Design

- A pure respelling: a single-faced card is written as an explicit call, with no implicit
  `CardFace` or `characteristics` wrap, and every card value stays the same.

## Done when

- Every single-faced card in `plugins/canon/` is written as an explicit call, and every face of a
  multi-faced card is written without the `characteristics` wrap.
- `cargo xtask dsl check` no longer reads a record at `Card`, or a `CardFace` record without
  `characteristics`, as the shorthand (`crates/xtask/src/dsl_check.rs`), and refuses each with a
  negative test.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check.rs` (the single-face exception), `plugins/canon/cards/`,
`docs/decisions/syl.md` §2 and §3.
