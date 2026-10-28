/-- Spell Swindle: "Counter target spell. If that spell is countered this way, create X Treasure
tokens, where X is that spell's mana value." [CR#608.2h] -/
card := {
  name := "Spell Swindle",
  cost := [3, blue, blue],
  types := [instant],
  text := [
    spell
      [
        counter (target isSpell),
        doIf (did any (counter (that spell)) thisWay)
          [create (letter x) treasureToken, define x (statOf (stat manaValue) (that spell))]
      ]
  ]
}
