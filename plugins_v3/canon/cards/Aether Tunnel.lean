card := {
  name := "Aether Tunnel",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [enchant isCreature, static [getsPt (enchanted creature) (up 1) (up 0), cant any (block it)]]
}
