card := {
  name := "Gratuitous Violence",
  cost := [2, red, red, red],
  types := [enchantment],
  text := [
    replace
      (would (a [isCreature, youControl]) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (instruct it (dealDamage (times 2 thatMuch))))
  ]
}
