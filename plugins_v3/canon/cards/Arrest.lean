card := {
  name := "Arrest",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        cant (enchanted creature) attack,
        cant (enchanted creature) block,
        cant any (activate (allOf [abilityHead anyActivated, abilityOf it]))
      ]
  ]
}
