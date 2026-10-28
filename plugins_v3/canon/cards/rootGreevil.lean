card := {
  name := "Root Greevil",
  cost := [3, green],
  types := [creature],
  subtypes := [beast],
  text := [
    activated (compound [[2, green], tapSymbol, sacrifice thisCreature])
      (destroy (allOf [isEnchantment, ofYourChoice color]))
  ],
  power := 2,
  toughness := 3
}
