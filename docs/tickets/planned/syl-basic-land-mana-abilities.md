---
needs: []
---

# The basic land subtypes confer their intrinsic mana abilities

## Why

An object with the land card type and a basic land type has the intrinsic ability "{T}: Add
[mana symbol]" [CR#305.6]. The five basic land subtypes
(`plugins/builtin/definitions/subtype/land/{plains,island,swamp,mountain,forest}.syl`) state it
only in a comment and confer nothing, so the rule lives only in the engine's name table
`BASIC_LAND_TYPES` (`crates/core/src/count.rs`, read by the engine's layers and counts).

## Done when

- `plains`, `island`, `swamp`, `mountain` and `forest` each confer their "{T}: Add [mana symbol]"
  ability [CR#305.6], conditioned on the object having the land card type.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Removing `BASIC_LAND_TYPES` from core and the engine: the engine reads core, which `.syl` reaches
only through the step-9 lowering.

## Pointers

`plugins/builtin/definitions/subtype/land/`, `plugins/builtin/definitions/card_type/creature.syl`
(a type's `confers`), `crates/core/src/count.rs`, `docs/decisions/semantics.md` §1 item 4.
