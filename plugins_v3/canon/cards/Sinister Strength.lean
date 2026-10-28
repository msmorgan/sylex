card := {
  name := "Sinister Strength",
  cost := [1, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [getsPt (enchanted creature) (up 3) (up 1), qualityChange it sets (colored (.some [black]))]
  ]
}
