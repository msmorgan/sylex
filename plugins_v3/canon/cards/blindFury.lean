/-- Blind Fury -/
card := {
  name := "Blind Fury",
  cost := [2, red, red],
  types := [instant],
  text := [
    spell
      [
        establishThroughout (abilityLoss (allOf isCreature) [written trample]) untilEndOfTurn,
        establishThroughout
          (replace (would (a isCreature) (dealDamage anyAmount (a isCreature) (some combat)))
             (instead (instruct it (dealDamage (times 2 thatMuch) (that creature)))))
          thisTurn
      ]
  ]
}
