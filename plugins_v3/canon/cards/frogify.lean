card := {
  name := "Frogify",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        allAbilityLoss (enchanted creature),
        qualityChange it sets (bundle { colors := [blue], types := [creature], subtypes := [frog] }),
        getsBase it (lit 1) (lit 1)
      ]
  ]
}
