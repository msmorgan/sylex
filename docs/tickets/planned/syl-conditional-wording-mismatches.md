---
needs: [cleanup-step6-surface]
---

# Conditional statics written in a wording their text does not print

## Why

These conditional statics are written in a shape or wording the rule or card text does not have,
so their text does not round-trip:

- `legendary` writes the [CR#205.4e] restriction under `asLongAs`, but the rule prints no "As
  long as": "Any instant or sorcery spell with the supertype "legendary" is subject to a casting
  restriction. A player can't cast …".
- `blitz` splits [CR#702.152a]'s one "As long as this permanent's blitz cost was paid, it has
  haste and 'When this permanent is put into a graveyard from the battlefield, draw a card.'"
  into two statics, each with its own condition. `offering` splits [CR#702.48a]'s one "If you
  chose to pay the additional cost, this spell's total cost is reduced …, and you may cast this
  spell any time you could cast an instant" the same way.
- `readAhead` writes `onlyWhile` around "the turn it entered the battlefield", and folds the
  printed "unless it has exactly the number of lore counters on it specified in the chapter
  symbol of that ability" ([CR#702.155a]) into the subject's predicate.
- *Jared Carthalion, True Heir* prints "If damage would be dealt to Jared Carthalion while you're
  the monarch, prevent …", but writes "while you're the monarch" as an `onlyWhile` condition
  around the whole replacement instead of inside its would-clause.
- *Pithing Needle* and *Damping Matrix* print "… can't be activated unless they're mana
  abilities", but fold the "unless" into the prohibited abilities' predicate
  (`not(isManaAbility)`).
- `partScope` has no `spelling`, and takes the part before whose, though "During your turn"
  prints whose first (`syl.md` §5); `duringPart`, its body, takes them in the same order.
- `onlyWhile` ("<spec> as long as <condition>") is named for "only while", which no card prints.

## Done when

- Each item above is written in the shape and wording its text prints, through a Template that
  spells it, or ticketed with the construct it lacks.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/supertype/legendary.syl`,
`plugins/builtin/definitions/keyword_ability/{blitz,offering,readAhead}.syl`,
`plugins/canon/cards/{Jared Carthalion, True Heir,Pithing Needle,Damping Matrix}.syl`,
`plugins/builtin/templates/ability/partScope.syl`,
`plugins/builtin/primitives/condition/duringPart.syl`,
`plugins/builtin/templates/static/onlyWhile.syl`.
