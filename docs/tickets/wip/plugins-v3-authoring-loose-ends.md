---
needs: [lean-generated-canon-modules]
---
# A loose end from the overnight `plugins-v3-*` chain

Standard constraints apply. These are Lean-only: the Rust crates are being rewritten
(`semantics-rust-rewrite`). The other items first listed here moved into
`plugins-v3-data-language`:

- the builtin bare-word rewrite, which the conversion script does;
- the `some`-wrapping list coercion;
- refusing wrongly typed locals (a bare word naming a parameter at a sort the parameter does not
  have), and the `up` pin.

Dropped outright: moving the dispatch bullet out of the ADR (§4.5 replaced that bullet with the
`.syl` dispatch rules); and the Rust filename test (it goes with the Rust crates); and requiring
`cardClass` on `card_type` (it defaults to `none`, which is the intended default).

1. **Delete `templates/ability/levelBand.lean` and `prototypeAlt.lean`.** They were kept only
   "until canon cleanup migrates their callers". Their callers are migrated, and both files
   still use `stat`.

## Acceptance

- The Rust expansion baseline (`cargo xtask expansions --compare`) still works until the
  rewrite. It shows only the two removed keys.
- Gate: `cargo xtask lean check`.
