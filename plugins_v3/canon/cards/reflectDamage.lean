card := {
  name := "Reflect Damage",
  cost := [3, red, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would (aYourChoice isSource) (dealDamage anyAmount))
            (instead (redirectDamage allDamage (controllerOf thatSource))) (use := nextTimeOnly))
         thisTurn)
  ]
}
