card := {
  name := "Pippin's Bravery",
  cost := [green],
  types := [instant],
  text := [
    spell
      (may (sacrifice (a food)) (get (target isCreature) (up 4) (up 4) untilEndOfTurn)
         (get (that creature) (up 2) (up 2) untilEndOfTurn))
  ]
}
