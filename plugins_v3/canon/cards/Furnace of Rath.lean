card := {
  name := "Furnace of Rath",
  cost := [1, red, red, red],
  types := [enchantment],
  text := [
    replace (would (a isSource) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (instruct it (dealDamage (times 2 thatMuch) thatJoin)))
  ]
}
