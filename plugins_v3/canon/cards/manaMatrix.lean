card := {
  name := "Mana Matrix",
  cost := [6],
  types := [artifact],
  text := [
    costShift (allOf [or [isInstant, isEnchantment], isSpell, did you (cast which) thisObject])
      (less 2)
  ]
}
