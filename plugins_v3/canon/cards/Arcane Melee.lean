card := {
  name := "Arcane Melee",
  cost := [4, blue],
  types := [enchantment],
  text := [costShift (allOf [isInstantOrSorcery, isSpell]) (less 2)]
}
