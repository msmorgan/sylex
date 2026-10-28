/-- Flickering Ward -/
card := {
  name := "Flickering Ward",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    entersChoosing thisAura color,
    retention (abilityGrant (enchanted creature) (protection (ofChosen color))) thisAura,
    activated [white] (returnToHand thisAura)
  ]
}
