---
needs: [syl-spelling-optional-segments]
---

# Say where a Tail stands among its phrase's adjuncts

## Why

A Tail attaches at the edge of an enclosing phrase, and the `spelling` notation names only that
phrase's Category (`docs/decisions/spelling.md` §3.2, §7). *Aurochs* prints "gets +1/+0 until
end of turn for each other attacking Aurochs" and *Ral's Staticaster* "gets +1/+0 for each card
in your hand until end of turn": the same Tail on the same verb phrase, outside the duration on
one card and inside it on the other. About a hundred in-scope cards print the first order after
a power/toughness change and a handful the second. With nothing to tell the placements apart,
both cards recover to one term and one of them does not round-trip.

## Design

- A Tail states its placement among the host phrase's other adjuncts: inside them, next to what
  the host itself prints, or outside them. Each placement a card prints is its own Template.

## Open questions

- Whether the placement needs its own mark in the notation. If the Readings of the two cards
  attach "for each …" to phrases of different Categories, naming the Category already says the
  placement. Recommendation: parse both lines first; use the Category if it separates them, and
  add a placement mark to the Tail only if it does not.

## Done when

- The Readings of the *Aurochs* and *Ral's Staticaster* lines are in the landing, with the
  Category and position at which "for each …" attaches in each.
- `docs/decisions/spelling.md` §3.2 and §7 state how a Tail's placement is written, and
  `sylex_syl` parses and formats it.
- Every "for each" and "equal to" wording that in-scope text prints in more than one placement
  has a Template for each; the landing lists them with a card for each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_syl` and
  `cargo test -p xtask` pass.

## Not in scope

- What "for each" scales in a power/toughness change: `syl-pt-change-sort`.

## Pointers

`docs/decisions/spelling.md` §3.2, §7, §10; `plugins/builtin/templates/amount/forEach.syl`;
`crates/english/grammar/english.fug` (the distributive adjunct's attachment).
