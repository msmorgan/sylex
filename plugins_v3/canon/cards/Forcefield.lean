card := {
  name := "Forcefield",
  cost := [3],
  types := [artifact],
  text := [
    activated [1]
      (establishThroughout
         (replace
            (would (aYourChoice [isCreature, isUnblocked]) (dealDamage anyAmount you (some combat)))
            (instead (preventDamage (allBut 1))) (use := nextTimeOnly))
         thisTurn)
  ]
}
