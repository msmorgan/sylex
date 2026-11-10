---
needs: [cleanup-step6-surface]
---

# A card type, subtype or supertype has no `name` beside its lemma

## Why

Every `card_type`, `subtype` and `supertype` declaration writes its word three times: as its
`lexical` lemma, as its `spelling` and as a `name` string (`card_type artifact` has
`lemma = "artifact"`, `spelling = "artifact"`, `name = "Artifact"`; `subtype urzas` has
`"Urza's"` three times). 477 declarations carry `name` (10 card types, 463 subtypes, 4
supertypes), nothing in `crates/` reads it, and the
`card_type`, `subtype` and `supertype` kinds in `crates/xtask/src/dsl_check/schema.rs` declare it.
A word with two sources can drift apart.

## Design

- A catalog declaration's word is its `lexical` lemma. The capitalized form a type line prints
  ("Artifact Creature") is orthography the English layer supplies, not a second field.
- `spelling` stays beside the lemma, though the two coincide on every catalog declaration: they
  are two layers of the one block (`docs/decisions/syl.md` §2), the lexeme the English layer
  inflects (`lexical`, with `plural = "Equipment"` where it is irregular) and the Construction's
  wording that spells the call (`docs/decisions/spelling.md` §1).

## Done when

- No `card_type`, `subtype` or `supertype` declaration writes `name`, and the three kinds have no
  `name` field.
- A token's `characteristics.name` (a characteristic, as *Walker*'s) is unchanged.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/{card_type,subtype,supertype}/`,
`crates/xtask/src/dsl_check/schema.rs`.
