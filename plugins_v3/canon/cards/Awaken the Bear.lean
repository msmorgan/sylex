card := {
  name := "Awaken the Bear",
  cost := [2, green],
  types := [instant],
  text := [
    spell
      (establishFor (target isCreature) [getsPt it (up 3) (up 3), abilityGrant it trample]
         untilEndOfTurn)
  ]
}
