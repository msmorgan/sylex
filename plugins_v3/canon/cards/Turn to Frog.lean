card := {
  name := "Turn to Frog",
  cost := [1, blue],
  types := [instant],
  text := [
    spell
      (establishThroughout
         [
           allAbilityLoss (target isCreature),
           qualityChange it sets (bundle { colors := [blue], subtypes := [frog] }),
           getsBase it 1 1
         ]
         untilEndOfTurn)
  ]
}
