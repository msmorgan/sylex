card := {
  name := "Reverse Damage",
  cost := [1, white, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would (aYourChoice isSource) (dealDamage anyAmount you))
            (instead (preventDamage allDamage (also := gainLife preventedThisWay)))
            (use := nextTimeOnly))
         thisTurn)
  ]
}
