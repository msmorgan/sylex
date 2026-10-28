card := {
  name := "Fiery Emancipation",
  cost := [3, red, red, red],
  types := [enchantment],
  text := [
    replace
      (would (a [isSource, youControl]) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (instruct it (dealDamage (times 3 thatMuch) thatJoin)))
  ]
}
