---
needs: [cleanup-step6-surface]
---

# A card type, subtype or supertype states its word once

## Why

Every `card_type`, `subtype` and `supertype` declaration writes its word three times: as its
`lexical` lemma, as its `spelling` and as a `name` string (`card_type artifact` has
`lemma = "artifact"`, `spelling = "artifact"`, `name = "Artifact"`; `subtype urzas` has
`"Urza's"` three times). 486 declarations carry `name`, nothing in `crates/` reads it, and the
`card_type`, `subtype` and `supertype` kinds in `crates/xtask/src/dsl_check/schema.rs` declare it.
A word with two sources can drift apart.

## Design

- A catalog declaration's word is its `lexical` lemma. The capitalized form a type line prints
  ("Artifact Creature") is orthography the English layer supplies, not a second field.

## Done when

- No `card_type`, `subtype` or `supertype` declaration writes `name`, and the three kinds have no
  `name` field.
- A token's `characteristics.name` (a characteristic, as *Walker*'s) is unchanged.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/{card_type,subtype,supertype}/`,
`crates/xtask/src/dsl_check/schema.rs`.
