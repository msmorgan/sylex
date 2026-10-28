card := {
  name := "Lightmine Field",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    whenever (does (counted (orMore 1) isCreature) attack)
      (instruct thisEnchantment
         (dealDamage (countOf [isCreature, isAttacking]) (eachOf (those creature))))
  ]
}
