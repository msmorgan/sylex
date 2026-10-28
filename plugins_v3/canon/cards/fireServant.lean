card := {
  name := "Fire Servant",
  cost := [3, red, red],
  types := [creature],
  subtypes := [elemental],
  text := [
    replace (would (a [isInstantOrSorcery, hasColor red, youControl]) (dealDamage anyAmount))
      (instead (instruct it (dealDamage (times 2 thatMuch))))
  ],
  power := 4,
  toughness := 3
}
