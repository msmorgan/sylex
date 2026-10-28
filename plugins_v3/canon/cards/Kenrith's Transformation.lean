card := {
  name := "Kenrith's Transformation",
  cost := [1, green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    when (does thisAura enter) (draw 1),
    static
      [
        allAbilityLoss (enchanted creature),
        qualityChange it sets (bundle { colors := [green], types := [creature], subtypes := [elk] }),
        getsBase it (lit 3) (lit 3)
      ]
  ]
}
