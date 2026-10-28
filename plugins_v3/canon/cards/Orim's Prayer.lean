card := {
  name := "Orim's Prayer",
  cost := [1, white, white],
  types := [enchantment],
  text := [
    whenever (does (counted (orMore 1) isCreature) (attack you))
      (gainLife (forEach 1 [isAttacking, isCreature]))
  ]
}
