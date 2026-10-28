card := {
  name := "Turn the Tables",
  cost := [3, white, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would any (dealDamage anyAmount you (some combat)))
            (instead (redirectDamage allDamage (target [isCreature, isAttacking]))))
         thisTurn)
  ]
}
