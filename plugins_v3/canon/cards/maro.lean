card := {
  name := "Maro",
  cost := [2, green, green],
  types := [creature],
  subtypes := [elemental],
  text := [ptDefinition thisCreature bothEach (countOf (cardIn yourHand))]
}
