---
needs: [syl-union-membership, syl-spelled-test-phrases]
specs: [kind-sorts-and-position-unions]
---

# Every word a card's term holds has a spelling

## Why

`docs/design/spelling.md` §7 says every word a card's term may hold has a `spelling`. Nothing
checks it, and it does not hold. All 981 canon card files call an unspelled declaration:
`singleFaced` (954 files), `static` (420), `actor` (409), `sequentially` (205), `ofChosen` (38),
`costShift` (36), `preventDamage` (29), `visibility` (20), `establish` (17). And 37 Holes of a
`spelling` are typed by a sort no declaration of which is spelled, so the wording they print
cannot be recovered: `hasPower` spells "with power $cmp $amt" over `Comparator`, where cards
print "with power 2", "with power 4 or greater" (*Elspeth, Sun's Champion*) and "with power
greater than the number of cards in your hand" (*Ensnaring Bridge*).

## Open questions

- **Which unspelled declarations are structure.** Some print nothing of their own and stand for
  structure the Reading has (`singleFaced` for the card, `static` for a sentence that is an
  ability, `perform` for a cost's verb phrase). Options: each takes a `spelling` that is its
  Hole alone; or a declaration may be marked as structure. For each of the unspelled words cards
  call, which it is, or the wording it lacks.
- **A parameter with no Hole.** `thatMuch(sort = …)` prints "that much" whatever `sort` is.
  Whether a card may pass an argument its wording does not print.

## Done when

- The open questions are answered in `docs/design/spelling.md` §7.
- `cargo xtask dsl check` refuses an unspelled declaration in a card's text and a `spelling` Hole
  whose sort has no spelled declaration, with each remaining exception named with the ticket
  that removes it.
- The 37 Holes are listed in the landing with the wording each takes, and `hasPower`,
  `hasManaValue` and `hasLesserManaValueThan` are the wordings cards print.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §6;
`docs/evidence/syl-kind-sorts/reframe-check.md` (the unspelled calls by declaration and the 37
Holes); `docs/design/spelling.md` §7.
