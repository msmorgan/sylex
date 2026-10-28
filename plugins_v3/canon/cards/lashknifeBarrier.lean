card := {
  name := "Lashknife Barrier",
  cost := [2, white],
  types := [enchantment],
  text := [
    when (does thisEnchantment enter) (draw 1),
    replace (would (a isSource) (dealDamage anyAmount (a [isCreature, youControl])))
      (instead (instruct it (dealDamage (minus thatMuch 1) (that creature))))
  ]
}
