---
needs: [cleanup-step6-surface]
---

# "Has a permanent type" reads `permanentType`

## Why

Each `card_type` declaration carries `permanentType: Bool` (`plugins/builtin/definitions/card_type/`,
true on the six types [CR#110.4]), yet `templates/predicate/isPermanentType.syl` lists the six
types again by hand, `or([hasType(artifact), …, hasType(planeswalker)])`. That is a second source
for one fact: a seventh permanent type, or a changed flag, would leave the Template stale.
Nothing in today's vocabulary can read the field: `hasType(type)` takes one type, and no
primitive quantifies over `card_type` declarations.

## Design

- A nullary Predicate primitive means "has a card type whose `permanentType` is true". It takes
  over `isPermanentType`'s name, its `TODO(spelling)` and its one caller, `isPermanentCard`
  ([CR#110.4a]), and the Template goes.

## Open questions

- How the field is read. `cargo xtask dsl check` reads only a catalog field's sort, never its
  value (`crates/xtask/src/dsl_check/schema.rs` declares `required("permanentType", "Bool")`, and
  no check reads a field value), and expansion substitutes Templates and wraps Definitions but
  computes nothing (`docs/decisions/syl.md` §4). The precedent is `hasSubtypesOf(token)`, an
  opaque primitive whose meaning reads a declaration's field after loading. Options: (a) the
  primitive stays opaque in `.syl` and the reading of the loaded catalog gives its meaning, so
  `dsl check` needs nothing new; (b) expansion rewrites it into the `or` of the flagged types,
  which makes expansion read catalog values, a new job `syl.md` §4 does not give it.
  Recommendation: (a), as `hasSubtypesOf` does.

## Done when

- No `.syl` file lists the six permanent types by hand; "has a permanent type" is one call of
  the new primitive.
- The primitive's doc comment says it reads `permanentType` and cites [CR#110.4].
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/predicate/{isPermanentType,isPermanentCard}.syl`,
`plugins/builtin/definitions/card_type/`, `plugins/builtin/primitives/predicate/{hasType,hasSubtypesOf}.syl`,
`crates/xtask/src/dsl_check/schema.rs`, `docs/decisions/syl.md` §4.
