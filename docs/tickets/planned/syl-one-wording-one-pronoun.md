---
needs: []
---

# Two pronoun Templates that print the same word

## Why

One wording recovers one written form (`docs/design/spelling.md`), and a term says what is
printed, not how it is read. Two pairs of `NounPhrase` Templates print the same word with
different bodies, so recovery has nothing to choose between them:

- `itself` (`refl(one)`) and `ownMember` (`pro(bare, one)`) both spell "itself". `ownMember`
  records a reading, the member a comparison over a domain is measuring.
- `them` (`pro(bare, many)`) and `themPlayers` (`pro(word(player), many)`) both spell "them".
  `themPlayers` records what the pronoun's antecedent is, which is Anaphor Resolution's to find
  (`docs/design/semantics.md` §2.4).

Nothing checks that two declarations of one sort print different words, so a third pair can be
added unnoticed.

## Done when

- "itself" has one written form and "them" has one written form, and the callers of `ownMember`
  and `themPlayers` write them; what each distinguished is read by semantics and stated where
  `semantics.md` discusses reflexives, measured members and Head Noun fit.
- Every other pair of declarations at one sort whose spellings recover the same words is listed
  from the bundle and resolved the same way, or shown to differ in what is printed (as `the` and
  `thePlural` differ in the noun's number).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/pronoun/{itself,ownMember,them,themPlayers}.syl`;
`docs/design/spelling.md` ("One wording recovers one written form", and the load rule on an
ambiguous skeleton); `docs/design/surface.md`; `docs/design/semantics.md` §2.2 item 5, §2.4.
