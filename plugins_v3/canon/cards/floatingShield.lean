/-- Floating Shield -/
card := {
  name := "Floating Shield",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    entersChoosing thisAura color,
    retention (abilityGrant (enchanted creature) (protection (ofChosen color))) thisAura,
    activated (sacrifice thisAura)
      (gain (target isCreature) (protection (ofChosen color)) untilEndOfTurn)
  ]
}
