/-- Ghoulflesh -/
card := {
  name := "Ghoulflesh",
  cost := [black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        getsPt (enchanted creature) (down 1) (down 1),
        qualityChange it adds (bundle { colors := [black], subtypes := [zombie] })
      ]
  ]
}
