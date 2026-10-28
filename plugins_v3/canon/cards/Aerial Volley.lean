card := {
  name := "Aerial Volley",
  cost := [green],
  types := [instant],
  text := [spell (dealDivided this_ 3 (targets (oneThrough 3) [isCreature, hasKeyword flying]))]
}
