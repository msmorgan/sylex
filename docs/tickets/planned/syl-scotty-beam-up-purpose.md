---
needs: []
---

# Author *Scotty, Miracle Worker*: "can't be beamed up to cast a spell this way"

## Why

*Scotty, Miracle Worker* reads "During your main phase, each card in your graveyard has beam me
up. The beam me up cost is equal to that spell's mana cost. Scotty can't be beamed up to cast a
spell this way." The purpose clause narrows the prohibition to beam me up costs paid for spells
cast through Scotty's own grant; a creature that can't be beamed up can't be returned to pay that
part of a beam me up cost [CR#702.196b], and the CR says nothing about a purpose. Every other
Vintage-legal purpose clause in Oracle text is on mana ("can't be spent to cast …",
`SpendPurpose`), so no `.syl` construction carries a purpose on a prohibited action, and Scotty
has no canon file. The English census already reads the sentence.

## Open questions

- A purpose parameter on `beamUp` alone ("beam <subject> up [to <purpose>]"), or a general purpose
  adjunct on Actions. Recommendation: the `beamUp` parameter, since no other card needs the
  general form.

## Done when

- `plugins/canon/cards/Scotty, Miracle Worker.syl` means its Oracle text: the main-phase grant of
  beam me up, its cost equal to the spell's mana cost, and the prohibition limited to spells cast
  this way.
- `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/beamMeUp.syl`,
  `plugins/builtin/definitions/keyword_action/beamUp.syl`
- `plugins/canon/cards/Marooned.syl` (the unqualified prohibition)
