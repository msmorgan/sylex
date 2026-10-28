card := {
  name := "Grievous Wound",
  cost := [3, black, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isPlayer,
    cant (enchanted player) (gainLife anyAmount),
    whenever (does any (dealDamage (to := enchanted player)))
      (instruct they (loseLife (half up (lifeTotalOf they))))
  ]
}
