card := {
  name := "Declaration of Naught",
  cost := [blue, blue],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment cardName,
    activated [blue] (counter (target [isSpell, named chosenName]))
  ]
}
