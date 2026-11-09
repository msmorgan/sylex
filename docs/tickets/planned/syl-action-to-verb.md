---
needs: []
---

# `Action` generalizes to a verb

## Why

*Guardian Beast* prints "This effect doesn't remove Auras already attached to those artifacts":
a negated verb whose subject is the whole effect. `.syl` writes it with the `retention` wrapper
around one clause of the effect (`plugins/builtin/primitives/static/retention.syl`), which is
wrong: the sentence is about the effect, not that clause. Which part of the effect it interacts
with ("can't be enchanted" making an Aura illegal [CR#704.5m]) is the engine's question.

## Open questions

- What `Action` generalizes to (a verb, or something similar), and how an effect stands as a
  verb's subject.

## Done when

- The generalization exists, *Guardian Beast* writes the sentence as a negated verb over the
  whole effect, and `retention` is gone.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/static/retention.syl`, `plugins/canon/cards/Guardian Beast.syl`,
`docs/decisions/semantics.md` §4.
