card := {
  name := "Pledge of Loyalty",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    retention
      (abilityGrant (enchanted creature)
         (protection (eachKind color (allOf [isPermanent, youControl]))))
      thisAura
  ]
}
