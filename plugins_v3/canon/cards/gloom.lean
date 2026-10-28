card := {
  name := "Gloom",
  cost := [2, black],
  types := [enchantment],
  text := [
    costShift (allOf [isSpell, hasColor white]) (more 3),
    costShift (allOf [abilityHead anyActivated, abilityOf (allOf [isEnchantment, hasColor white])])
      (more 3)
  ]
}
