---
needs: []
---

# Consistent whitespace rules for `< … >` optionals and `#<…>` list joins in spelling notation

## Why

In `sylex_spelling`, angle-bracketed `< … >` optional parts treat spaces inside the angle brackets
as significant literal text (`"put $subject into $destination< from $from>"` relies on the leading
space inside `< from $from>`, whereas `docs/design/spelling.md` §7's grammar comment writes
`< … >` with padding spaces). By contrast, `#<join>` list joins match exact unpadded tokens
(`#<and>`, `#<,>`, `#<.>`, `#<>`, `#<\n>`) and reject `#< and >` as an unknown join while
`docs/design/spelling.md` §7 also writes `#< … >` in prose. Using `<…>` delimiters with opposite
whitespace conventions invites off-by-one space bugs in authored `spelling` strings.

## Open questions

- How `<…>` optional parts and `#<…>` list joins should treat leading and trailing spaces inside
  their angle brackets (for example, whether word boundaries between adjacent segments are derived
  by spelling so `<…>` trims delimiter padding, or `#<…>` stays closed-token syntax while
  `spelling.md` §7 drops the padded `#< … >` / `< … >` metanotation, or optional parts use a
  distinct delimiter).

## Done when

- The open question is answered and `docs/design/spelling.md` §7 states one unambiguous rule for
  whitespace inside and around optional parts and list joins.
- `crates/spelling`, `crates/xtask/src/dsl_check/spelling.rs` and every affected `spelling` under
  `plugins/` conform to that rule, with unit tests covering boundary whitespace cases.
- `cargo test -p sylex_spelling`, `cargo test -p xtask`, `cargo clippy -p sylex_spelling -p xtask --tests -- -D warnings`,
  `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Pointers

`crates/spelling/src/notation.rs`, `crates/xtask/src/dsl_check/spelling.rs`,
`docs/design/spelling.md` §7.
