card := {
  name := "Benevolent Blessing",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    flash,
    enchant isCreature,
    entersChoosing thisAura color,
    retention (abilityGrant (enchanted creature) (protection (ofChosen color)))
      (allOf [or [aura, equipment], youControl, attachedTo it])
  ]
}
