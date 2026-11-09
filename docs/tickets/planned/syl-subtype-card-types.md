---
needs: [syl-primitive-constructors]
---

# A subtype names the card types it is correlated with

## Why

A `subtype` declaration's `kind = spell` is matched as text in `lean/SylGen/Emit.lean`, a word
that never resolves. Behind it, Lean splits subtypes into `Subtype.of(host)` and `Subtype.spell`,
which models the CR wrongly: each subtype is correlated to card types [CR#205.3c,205.3d], and two
families are shared, spell types by instants and sorceries [CR#205.3k] and creature types by
creatures and kindreds [CR#205.3m]. So kindred needs a hardcoded special case in the checker
(`subsFitLine`, `lean/Semantics/Check/Words.lean`).

## Design

- A `subtype` declares the card types it is correlated with, as a `List(CardType)`.
- The two shared families are parameterless Templates at `List(CardType)`: `creature` is
  `[creature, kindred]` and `spell` is `[instant, sorcery]`, so `kind = creature` and
  `kind = spell` resolve as written.
- A single-type family writes a list literal (`[battle]`): a `battle` Template would form a
  cycle with `siege` through `card_type battle`'s `confers`.
- `Subtype` is one shape with no `spell` constructor, the shape `sylex_core::Subtype.types`
  (`crates/core/src/type.rs`) already has.

## Done when

- The `kind = spell` text match in `Emit.lean`, `Subtype.spell` and the kindred special case in
  `subsFitLine` are deleted.
- Every word at a subtype's `kind` resolves through a declaration.
- Values change only in each subtype's card-type list: `lake exe dumpValues compare` shows no
  other changed row, and the landing reports what changed.
- `cargo xtask lean check`, `cargo xtask dsl fmt --check`, `cargo xtask dsl differential` and
  `cargo test -p sylex_syl` pass.

## Pointers

`lean/SylGen/Emit.lean` (the `subtype` kind), `lean/Semantics/Check/Words.lean` (`subsFitLine`),
`plugins/builtin/definitions/subtype/`, `plugins/builtin/definitions/card_type/battle.syl`,
`crates/core/src/type.rs`.
