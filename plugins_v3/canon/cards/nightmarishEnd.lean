/-- Nightmarish End -/
card := {
  name := "Nightmarish End",
  cost := [2, black],
  types := [instant],
  text := [
    spell
      [
        get (target isCreature) (down (letter x)) (down (letter x)) untilEndOfTurn,
        define x (countOf (cardIn yourHand))
      ]
  ]
}
