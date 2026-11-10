---
needs: [cleanup-step6-surface]
---

# Referents a conditional's consequent introduces that later text reads

## Why

`semantics.md` §2.2 item 2 exports from a conditional Static Spec's consequent only the plural
domain of a bare plural or "all". Two Vintage-legal cards read other referents their conditional's
consequent introduces:

- *Return the Past*: "During your turn, each instant and sorcery card in your graveyard has
  flashback. Its flashback cost is equal to its mana cost." "Its" reads the distributive "each
  instant and sorcery card", one card at a time, from a sentence after the conditional one.
- *Share the Spoils*: "During each player's turn, that player may play a land or cast a spell from
  among cards exiled with this enchantment, and they may spend mana as though it were mana of any
  color to cast that spell. When they do, exile the top card of their library." "they" and "do"
  in the last sentence read the player and the play or cast of the conditional's consequent.

## Open questions

- Is the export rule widened to these referents (a distributive "each" member, an optional
  action and its performer), or are such sentences authored inside the consequent, as part of the
  one Static Spec the condition governs?

## Done when

- *Return the Past* and *Share the Spoils* are written in `plugins/canon/cards/`, and §2.2 states
  how their later sentence reaches what it reads.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/semantics.md` §2.2; `plugins/canon/cards/Guardian Beast.syl` (the export rule's
one user).
