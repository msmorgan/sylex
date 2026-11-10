---
needs: [cleanup-step6-surface]
---

# State which objects a per-holder `stateBased` conferral applies to

## Why

A `stateBased(when, then)` entry in a declaration's `confers` is enabled for each holder, and
nothing states which holders: a card in a hand holds its card types and supertypes too. As
written, a world card in hand matches `world`'s entry, a 0/0 creature card in hand (*Walking
Ballista*) matches `creature`'s toughness entry, and a planeswalker card in hand, which has no
loyalty counters, matches `planeswalker`'s loyalty entry. Each of those state-based actions is
about permanents: "two or more permanents" [CR#704.5k], and "a creature" and "a planeswalker"
[CR#704.5f,704.5i], which mean permanents [CR#109.2].

## Open questions

- State it once, on the `stateBased` primitive (a `stateBased` conferral applies only to
  permanents), or guard each entry with `is(this, isPermanent)`?

## Done when

- Which holders a `stateBased` conferral applies to is stated where the owner's answer puts it,
  and every `stateBased` entry under `plugins/builtin/definitions/` reads correctly under it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/ability/stateBased.syl`, `plugins/builtin/definitions/card_type/`,
`plugins/builtin/definitions/subtype/`, `plugins/builtin/definitions/supertype/world.syl`.
