---
needs: [lean-generated-canon-modules]
---
# Two loose ends from the overnight `plugins-v3-*` chain

Standard constraints apply. These are Lean-only: the Rust crates are being rewritten
(`semantics-rust-rewrite`). The other items first listed here moved into
`plugins-v3-data-language`:

- the builtin bare-word rewrite, which the conversion script does;
- the `some`-wrapping list coercion;
- refusing wrongly typed locals;
- the `up` pin;
- moving the dispatch bullet out of the ADR.

One more was dropped outright: the Rust filename test, which goes with the Rust crates.

1. **Delete `templates/ability/levelBand.lean` and `prototypeAlt.lean`.** They were kept only
   "until canon cleanup migrates their callers". Their callers are migrated, and both files
   still use `stat`.
2. **Make `cardClass` required on `card_type`.**
   - `cardClass` defaults to `none`, so a permanent type that leaves it out silently has no Card
     Class.
   - Require it in Lean `card_type` authoring. `none` must be written explicitly (kindred
     [CR#308.1], dungeon [CR#309.2c]).
   - Add a negative pin.

## Acceptance

- The Rust expansion baseline (`cargo xtask expansions --compare`) still works until the
  rewrite. It shows only item 1's two removed keys.
- Gate: `cargo xtask lean check`.
