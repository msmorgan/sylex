card := {
  name := "Elemental Resonance",
  cost := [2, green, green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isPermanent,
    at_ (beginningOfYour firstMain) (addMana 1 (matchingManaCostOf (enchanted permanent)))
  ]
}
