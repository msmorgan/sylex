card := {
  name := "Rustler Rampage",
  cost := [white],
  types := [instant],
  text := [
    spell
      (chooseSpree
         [
           { cost := [1], action := untap (allOf [isCreature, controlledBy (target isPlayer)]) },
           { cost := [1], action := gain (target isCreature) doubleStrike untilEndOfTurn }
         ])
  ]
}
