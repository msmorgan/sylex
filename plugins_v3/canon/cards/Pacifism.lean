/-- Pacifism -/
card := {
  name := "Pacifism",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [enchant isCreature, static [cant (enchanted creature) attack, cant (enchanted creature) block]]
}
