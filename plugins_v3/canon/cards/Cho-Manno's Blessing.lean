card := {
  name := "Cho-Manno's Blessing",
  cost := [white, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    flash,
    enchant isCreature,
    entersChoosing thisAura color,
    retention (abilityGrant (enchanted creature) (protection (ofChosen color))) thisAura
  ]
}
