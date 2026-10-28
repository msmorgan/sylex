card := {
  name := "Nevermore",
  cost := [1, white, white],
  types := [enchantment],
  text := [
    entersChoosingFrom thisEnchantment cardName (nameOfCard (non isLand)),
    cant any (cast (bare [isSpell, named chosenName]))
  ]
}
