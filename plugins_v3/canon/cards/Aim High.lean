card := {
  name := "Aim High",
  cost := [1, green],
  types := [instant],
  text := [
    spell
      [
        untap (target isCreature),
        establishFor it [getsPt it (up 2) (up 2), abilityGrant it reach] untilEndOfTurn
      ]
  ]
}
