card := {
  name := "Dazzling Reflection",
  cost := [1, white],
  types := [instant],
  text := [
    spell
      [
        gainLife (powerOf (target isCreature)),
        establishThroughout
          (replace (would (that creature) (dealDamage anyAmount))
             (instead (preventDamage allDamage)) (use := nextTimeOnly))
          thisTurn
      ]
  ]
}
