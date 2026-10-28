card := {
  name := "Borrowed Malevolence",
  cost := [black],
  types := [instant],
  text := [
    escalate [2],
    spell
      (chooseModes (oneThrough 2)
         [
           get (target isCreature) (up 1) (up 1) untilEndOfTurn,
           get (target isCreature) (down 1) (down 1) untilEndOfTurn
         ])
  ]
}
