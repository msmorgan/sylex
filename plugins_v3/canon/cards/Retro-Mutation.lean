/-- Retro-Mutation -/
card := {
  name := "Retro-Mutation",
  cost := [2, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    flash,
    enchant isCreature,
    static
      [
        qualityChange (enchanted creature) sets (bundle { subtypes := [turtle] }),
        getsBase it 0 1,
        cant it attack,
        allAbilityLoss it
      ]
  ]
}
