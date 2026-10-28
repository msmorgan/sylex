card := {
  name := "Stupefying Touch",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    when (does thisAura enter) (draw 1),
    cant any (activate (allOf [abilityHead anyActivated, abilityOf (enchanted creature)]))
  ]
}
