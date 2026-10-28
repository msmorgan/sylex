/-- Leonin Bola -/
card := {
  name := "Leonin Bola",
  cost := [1],
  types := [artifact],
  subtypes := [equipment],
  text := [
    abilityGrant (equipped creature)
      (activated (compound [tapSymbol, unattach (theGrantor permanent)]) (tap (target isCreature))),
    equip [1]
  ]
}
