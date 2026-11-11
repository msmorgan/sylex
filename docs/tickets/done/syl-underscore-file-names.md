---
needs: [cleanup-step6-spine]
---

# File names without a Lean-keyword underscore

## Why

Seven `.syl` files end in an underscore that only kept their names off Lean keywords:
`primitives/action/repeat_.syl` (`repeat`), `primitives/noun/by_.syl` (`by`),
`primitives/predicate/in_.syl` (`in`), `primitives/pronoun/this_.syl` (`this`),
`templates/determiner/exists_.syl` (`exists`), `definitions/subtype/creature/seal_.syl` (`seal`)
and `definitions/subtype/enchantment/class_.syl` (`class`), all under `plugins/builtin/`. The
declarations inside already carry the plain names, and paths are for people
(`docs/decisions/syl.md` §2), so the underscore tells a reader nothing.

## Done when

- Each of the seven files is named for the declaration it holds, with no trailing underscore, and
  no other file under `plugins/` has one.
- Nothing outside `docs/tickets/done/` names an old path.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/syl.md` §2 ("Paths are for people").
