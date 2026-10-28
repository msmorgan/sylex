card := {
  name := "Circle of Protection: Red",
  cost := [1, white],
  types := [enchantment],
  text := [
    activated [1]
      (establishThroughout
         (replace (would (aYourChoice [isSource, hasColor red]) (dealDamage anyAmount you))
            (instead (preventDamage allDamage)) (use := nextTimeOnly))
         thisTurn)
  ]
}
