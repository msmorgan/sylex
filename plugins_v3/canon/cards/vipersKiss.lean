card := {
  name := "Viper's Kiss",
  cost := [black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        getsPt (enchanted creature) (down 1) (down 1),
        cant any (activate (allOf [abilityHead anyActivated, abilityOf it]))
      ]
  ]
}
