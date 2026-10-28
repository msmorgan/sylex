/-- Festival -/
card := {
  name := "Festival",
  cost := [white],
  types := [instant],
  text := [
    spell (establishThroughout (cant (allOf isCreature) attack) thisTurn)
      (timing := duringPart upkeep anOpponent)
  ]
}
