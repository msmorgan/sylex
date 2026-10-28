card := {
  name := "Rustler Rampage",
  cost := [white],
  types := [instant],
  text := [
    spell
      (chooseSpree
         [
           ([1], untap (allOf [isCreature, controlledBy (target isPlayer)])),
           ([1], gain (target isCreature) doubleStrike untilEndOfTurn)
         ])
  ]
}
