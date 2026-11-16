---
needs: [syl-spelling-optional-segments]
---

# Say where a Tail stands among its phrase's adjuncts

## Why

A Tail prints at the edge of the phrase that hosts it. A trailing Tail stands inside the fronted
dependents of its Category; unmarked, it stands outside the host's other trailing dependents,
and marked `inner` it stands before them (`docs/design/spelling.md`, on Tails and the
notation). No declaration uses the mark yet. *Aurochs* prints "gets +1/+0 until
end of turn for each other attacking Aurochs" and *Ral's Staticaster* "gets +1/+0 for each card
in your hand until end of turn": the same Tail on the same verb phrase, outside the duration on
one card and inside it on the other. About a hundred in-scope cards print the first order after
a power/toughness change and a handful the second. With one Template for both, the two cards
recover to one term and one of them does not round-trip.

## Design

- A Tail states its placement among the host's other trailing dependents: before them (`inner`)
  or after them (unmarked). Each placement a card prints is its own Template.

## Done when

- Every `spelling` this ticket writes parses with `sylex_spelling`, the ones marked `inner`
  among them, and `forEach` is spelled with its Tail, "$per %VerbPhrase: for each $p".
- Every "for each" and "equal to" wording that in-scope text prints in more than one placement
  has a Template for each; the landing lists them with a card for each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_spelling` and
  `cargo test -p xtask` pass.

## Not in scope

- Finding a Tail's host in a Reading: recovery's (`english-syl-term-recovery`).
- What "for each" scales in a power/toughness change: `syl-pt-change-sort`.
- The Tails of a copy's ", except …" (`syl-copy-exceptions`) and of `half`'s ", rounded …"
  (`syl-round-each-time`), each written by its own ticket under the placement rule above.

## Pointers

`docs/design/spelling.md` (Tails, the notation); `crates/spelling/`;
`plugins/builtin/templates/amount/forEach.syl`;
`crates/english/grammar/english.fug` (the distributive adjunct's attachment).
