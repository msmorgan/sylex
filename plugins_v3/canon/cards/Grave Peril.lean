card := {
  name := "Grave Peril",
  cost := [1, black],
  types := [enchantment],
  text := [
    when (does (a [isCreature, non (hasColor black)]) enter)
      (doIfDone (sacrifice thisEnchantment) (destroy (that (type creature))))
  ]
}
