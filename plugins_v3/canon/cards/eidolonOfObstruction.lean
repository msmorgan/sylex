card := {
  name := "Eidolon of Obstruction",
  cost := [1, white],
  types := [enchantment, creature],
  subtypes := [spirit],
  text := [
    firstStrike,
    costShift (allOf [abilityHead loyalty, abilityOf (allOf [isPlaneswalker, opponentsControl])])
      (more 1)
  ],
  power := 2,
  toughness := 1
}
