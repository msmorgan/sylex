card := {
  name := "Lignify",
  cost := [1, green],
  types := [kindred, enchantment],
  subtypes := [treefolk, aura],
  text := [
    enchant isCreature,
    static
      [
        qualityChange (enchanted creature) sets (bundle { subtypes := [treefolk] }),
        getsBase it 0 4,
        allAbilityLoss it
      ]
  ]
}
