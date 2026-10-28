/-- Pentarch Ward -/
card := {
  name := "Pentarch Ward",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    entersChoosing thisAura color,
    when (does thisAura enter) (draw 1),
    retention (abilityGrant (enchanted creature) (protection (ofChosen color))) thisAura
  ]
}
