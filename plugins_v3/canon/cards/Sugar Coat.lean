card := {
  name := "Sugar Coat",
  cost := [2, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    flash,
    enchant (or [isCreature, food]),
    static
      [
        qualityChange (enchanted permanent) sets
          (bundle
             {
               types := [artifact],
               subtypes := [food],
               text := [activated (compound [[2], tapSymbol, (sacrifice thisArtifact)]) (gainLife 3)]
             }),
        allAbilityLoss it
      ]
  ]
}
