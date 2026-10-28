card := {
  name := "Lava Axe",
  cost := [4, red],
  types := [sorcery],
  text := [spell (instruct this_ (dealDamage 5 (target (or [isPlaneswalker, isPlayer]))))]
}
