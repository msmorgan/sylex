card := {
  name := "Teleport",
  cost := [blue, blue, blue],
  types := [instant],
  text := [
    spell (establishThroughout (cant any (block (target isCreature))) thisTurn)
      (timing := duringPart declareAttackers none)
  ]
}
