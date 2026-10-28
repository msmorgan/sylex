card := {
  name := "Accelerated Mutation",
  cost := [3, green, green],
  types := [instant],
  text := [
    spell
      [
        get (target isCreature) (up (letter x)) (up (letter x)) untilEndOfTurn,
        define x (aggregate max (stat manaValue) [isPermanent, youControl])
      ]
  ]
}
