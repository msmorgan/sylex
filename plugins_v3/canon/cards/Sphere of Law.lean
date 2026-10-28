card := {
  name := "Sphere of Law",
  cost := [3, white],
  types := [enchantment],
  text := [
    replace (would (a [isSource, hasColor red]) (dealDamage anyAmount you))
      (instead (preventDamage (ofThatDamage 2)))
  ]
}
