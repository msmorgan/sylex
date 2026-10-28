card := {
  name := "Searing Flesh",
  cost := [6, red],
  types := [sorcery],
  text := [spell (instruct this_ (dealDamage 7 (target (or [isPlaneswalker, isOpponent]))))]
}
