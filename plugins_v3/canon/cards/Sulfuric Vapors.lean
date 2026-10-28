card := {
  name := "Sulfuric Vapors",
  cost := [3, red],
  types := [enchantment],
  text := [
    replace
      (would (a [isSpell, hasColor red]) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (instruct it (dealDamage (plus thatMuch 1) thatJoin)))
  ]
}
