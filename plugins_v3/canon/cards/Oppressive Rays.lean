card := {
  name := "Oppressive Rays",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static [gatedBy [3] (enchanted creature) attack, gatedBy [3] (enchanted creature) block],
    costShift (allOf [abilityHead anyActivated, abilityOf (enchanted creature)]) (more 3)
  ]
}
