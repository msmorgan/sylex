card := {
  name := "Aegis of Honor",
  cost := [white],
  types := [enchantment],
  text := [
    activated [1]
      (establishThroughout
         (replace (would (a isInstantOrSorcery) (dealDamage anyAmount you))
            (instead (redirectDamage allDamage (controllerOf it))) (use := nextTimeOnly))
         thisTurn)
  ]
}
