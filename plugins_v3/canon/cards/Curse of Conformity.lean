/-- Curse of Conformity -/
card := {
  name := "Curse of Conformity",
  cost := [4, white],
  types := [enchantment],
  subtypes := [aura, curse],
  text := [
    enchant isPlayer,
    static
      [
        getsBase (allOf [isCreature, non (hasSupertype legendary), controlledBy (enchanted player)])
          3 3,
        qualityChange them loses (everyTypeOf creature)
      ]
  ]
}
