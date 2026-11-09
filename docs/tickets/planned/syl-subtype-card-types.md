---
needs: [syl-primitive-constructors]
---

# A subtype names the card types it is correlated with

## Why

A `subtype` declaration's `kind = spell` is a word that never resolves: `cargo xtask dsl check`
accepts it as text. Behind it is a model that splits subtypes into one host card type or `spell`,
which models the CR wrongly: each subtype is correlated to card types [CR#205.3c,205.3d], and two
families are shared, spell types by instants and sorceries [CR#205.3k] and creature types by
creatures and kindreds [CR#205.3m]. Under that split, kindred needs a special case.

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

- Every word at a subtype's `kind` resolves through a declaration.
- `cargo xtask dsl check` has no `Kind.text_word` exception
  (`crates/xtask/src/dsl_check/schema.rs`), so a subtype's `kind` always resolves at its sort.
- `Subtype` has one shape: the validator's `subtype` kind declares `kind` at `List(CardType)`,
  nothing declares or reserves a `spell` constructor at `Subtype`, and `cargo xtask dsl check`
  refuses a subtype whose `kind` is a single card type, with a negative test.
- The `.syl` changes are each subtype's `kind`, the `creature` and `spell` Templates, and
  `card_type/battle.syl` where the cycle needs it; the landing lists them.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check/schema.rs` (`text_word`), `plugins/builtin/definitions/subtype/`,
`plugins/builtin/definitions/card_type/battle.syl`, `crates/core/src/type.rs`.
