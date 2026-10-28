card := {
  name := "Efflorescence",
  cost := [2, green],
  types := [instant],
  text := [
    spell (putCounters 2 p1p1Counter (target isCreature)),
    infusion
      (spell
         (doIf (did you (gainLife anyAmount) soFarThisTurn)
            (establishFor (that creature) [abilityGrant it trample, abilityGrant it indestructible]
               untilEndOfTurn)))
  ]
}
