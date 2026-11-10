---
needs: [cleanup-step6-spine]
---

# A Template for mana printed "for each" with no count

## Why

*Gaea's Cradle* prints "Add {G} for each creature you control", with no number before the
symbol. Its term is `counted(forEach(1, …), symbols([pip(green)]))`, which stores a 1 the card
does not print, and `counted`'s `TODO(spelling)` asks spelling to drop it. The notation has no
omission governed by a value (`docs/decisions/spelling.md` §7), so the printed form is its own
Template (§3.2).

## Design

- `forEach` is declared at `ProducedMana`: it takes the produced mana whole and a Predicate,
  is spelled as the mana followed by a "for each" Tail, and expands to the `counted` term with
  a per of 1. A choice inside the mana is still made once per mana ("{B} or {G} for each
  permanent destroyed this way"), as `counted`'s expansion already has it.
- This is a known weak point, kept for now: each sort scaled whole needs its own "for each"
  Template. The landing lists every other sort where in-scope text prints "for each" beside
  something with no count of its own, so the next decision has the evidence.

## Done when

- The Template exists, *Gaea's Cradle* and every other canon card printing mana "for each" with
  no count call it, and no term in `plugins/` writes `counted(forEach(1, …), …)`.
- `counted`'s `TODO(spelling)` no longer asks for a per of one to print nothing.
- The landing's list of other sorts scaled whole, with a card for each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- "An amount of {G} equal to …", which is another wording and keeps its `TODO(spelling)`.
- "For each" over a power/toughness change: `syl-pt-change-sort`.

## Pointers

`docs/decisions/spelling.md` §3.2, §7; `plugins/builtin/primitives/producedMana/counted.syl`,
`plugins/builtin/templates/amount/forEach.syl`, `plugins/canon/cards/Gaea's Cradle.syl`.
