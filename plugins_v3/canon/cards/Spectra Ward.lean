card := {
  name := "Spectra Ward",
  cost := [3, white, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    conjunction
      [
        getsPt (enchanted creature) (up 2) (up 2),
        retention (abilityGrant (enchanted creature) (protection (eachKind color))) (allOf aura)
      ]
      (subject := enchanted creature)
  ]
}
