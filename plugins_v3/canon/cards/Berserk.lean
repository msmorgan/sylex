card := {
  name := "Berserk",
  cost := [green],
  types := [instant],
  text := [
    spell
      [
        establishFor (target isCreature)
          [
            abilityGrant it trample,
            getsPt it (up (letter x)) (up 0),
            letterDefinition x (powerOf it)
          ]
          untilEndOfTurn,
        atNext endStep
          (doOnlyIf (destroy (that creature)) (did (that creature) attack soFarThisTurn))
      ]
      (timing := beforePart combatDamage none)
  ]
}
