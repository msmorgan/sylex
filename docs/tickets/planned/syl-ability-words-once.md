---
needs: []
---

# Ability words exist once

## Why

Ability words are declared twice: as `ability_word` declarations under
`plugins/builtin/templates/ability_word/`, and through an `abilityWord` Template
(`plugins/builtin/templates/ability/abilityWord.syl`) over the `abilityWord` italic-word primitive,
which nothing uses. The validator also keeps `AbilityWordLabel` as a sort no declaration produces
(`UNDECLARED_SORTS`, `crates/xtask/src/dsl_check/schema.rs`).

## Done when

- One form declares ability words; the other is deleted, with every caller using the survivor.
- `AbilityWordLabel` is produced by a declaration or gone, and `UNDECLARED_SORTS` and its "Step 6"
  note no longer list it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/ability_word/`, `plugins/builtin/templates/ability/abilityWord.syl`,
`plugins/builtin/primitives/italicWord/abilityWord.syl`, `crates/xtask/src/dsl_check/schema.rs`.
