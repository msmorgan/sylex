card := {
  name := "Training Grounds",
  cost := [blue],
  types := [enchantment],
  text := [
    costShift (allOf [abilityHead anyActivated, abilityOf (allOf [isCreature, youControl])])
      (less 2 (some 1))
  ]
}
