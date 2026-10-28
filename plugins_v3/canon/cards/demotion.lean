card := {
  name := "Demotion",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        cant (enchanted creature) block,
        cant any (activate (allOf [abilityHead anyActivated, abilityOf it]))
      ]
  ]
}
