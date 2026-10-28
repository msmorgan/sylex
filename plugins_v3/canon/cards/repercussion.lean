card := {
  name := "Repercussion",
  cost := [1, red, red],
  types := [enchantment],
  text := [
    whenever (does any (dealDamage (to := a isCreature)))
      (instruct thisEnchantment (dealDamage thatMuch (controllerOf (that creature))))
  ]
}
