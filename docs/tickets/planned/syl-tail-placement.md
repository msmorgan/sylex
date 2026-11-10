---
needs: [syl-spelling-optional-segments]
---

# Say where a Tail stands among its phrase's adjuncts

## Why

A Tail attaches at the edge of an enclosing phrase, inside or outside that phrase's other
adjuncts, and the notation marks the inside placement `inner` (`docs/decisions/spelling.md`
§3.2, §7). No declaration uses the mark yet. *Aurochs* prints "gets +1/+0 until
end of turn for each other attacking Aurochs" and *Ral's Staticaster* "gets +1/+0 for each card
in your hand until end of turn": the same Tail on the same verb phrase, outside the duration on
one card and inside it on the other. About a hundred in-scope cards print the first order after
a power/toughness change and a handful the second. With one Template for both, the two cards
recover to one term and one of them does not round-trip.

## Design

- A Tail states its placement among the host phrase's other adjuncts: inside them, next to what
  the host itself prints, or outside them. Each placement a card prints is its own Template.

## Open questions

- Whether the two placements are told apart by `inner` or by Category. If the Readings of the
  two cards attach "for each …" to phrases of different Categories, naming the Category already
  says the placement. Recommendation: parse both lines first; name the Category if it separates
  them, and use `inner` where it does not.

## Done when

- The Readings of the *Aurochs* and *Ral's Staticaster* lines are in the landing, with the
  Category and position at which "for each …" attaches in each.
- `sylex_syl` parses and formats `inner`, and resolving a Tail's host follows
  `docs/decisions/spelling.md` §7.
- Every "for each" and "equal to" wording that in-scope text prints in more than one placement
  has a Template for each; the landing lists them with a card for each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_syl` and
  `cargo test -p xtask` pass.

## Not in scope

- What "for each" scales in a power/toughness change: `syl-pt-change-sort`.

## Pointers

`docs/decisions/spelling.md` §3.2, §7; `plugins/builtin/templates/amount/forEach.syl`;
`crates/english/grammar/english.fug` (the distributive adjunct's attachment).
