---
needs: []
---

# Expressions copied for lack of a local binding

## Why

`.syl` has no local bindings (`docs/decisions/syl.md` §2), so a body that needs one value twice
writes the expression twice: melee, rampage and sunburst copy theirs. `theActor` carries
`identity = "actor"`, a field on the `template` kind (`crates/xtask/src/dsl_check/schema.rs`, marked
"Step 6") that stands in for a binding.

## Open questions

- Whether `.syl` gains a local binding, against §2's "no local bindings", or the copies go some
  other way (a Template, a Definition parameter).
- What replaces `identity = "actor"`.

## Done when

- Melee, rampage and sunburst write each value once, as settled.
- The `template` kind has no `identity` field, and `theActor` reads as settled.
- `docs/decisions/syl.md` §2 states the rule.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/keyword_ability/{melee,rampage,sunburst}.syl`,
`plugins/builtin/templates/pronoun/theActor.syl`, `crates/xtask/src/dsl_check/schema.rs`
(`template`), `docs/decisions/syl.md` §2.
