card := {
  name := "Urza's Armor",
  cost := [6],
  types := [artifact],
  text := [
    replace (would (a isSource) (dealDamage anyAmount you))
      (instead (preventDamage (ofThatDamage 1)))
  ]
}
