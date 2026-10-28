card := {
  name := "Volatile Claws",
  cost := [2, red],
  types := [instant],
  text := [
    spell
      (establishFor (allOf [isCreature, youControl])
         [getsPt them (up 2) (up 0), qualityChange them adds (everyTypeOf creature)] untilEndOfTurn)
  ]
}
