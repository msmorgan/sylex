card := {
  name := "Chance for Glory",
  cost := [1, red, white],
  types := [instant],
  text := [
    spell
      [
        establish (abilityGrant (allOf [isCreature, youControl]) indestructible),
        extraTurn 1,
        delay (beginningOfThatTurns endStep) loseGame
      ]
  ]
}
