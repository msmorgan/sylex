card := {
  name := "Nylea's Presence",
  cost := [1, green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isLand,
    when (does thisAura enter) (draw 1),
    qualityChange (enchanted land) adds (everyTypeOf basicLand)
  ]
}
