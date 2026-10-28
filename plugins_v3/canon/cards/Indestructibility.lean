/-- Indestructibility -/
card := {
  name := "Indestructibility",
  cost := [3, white],
  types := [enchantment],
  subtypes := [aura],
  text := [enchant isPermanent, abilityGrant (enchanted permanent) indestructible]
}
