card := {
  name := "Sphere of Purity",
  cost := [3, white],
  types := [enchantment],
  text := [
    replace (would (a isArtifact) (dealDamage anyAmount you))
      (instead (preventDamage (ofThatDamage 1)))
  ]
}
