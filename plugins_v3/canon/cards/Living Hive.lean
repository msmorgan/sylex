card := {
  name := "Living Hive",
  cost := [6, green, green],
  types := [creature],
  subtypes := [elemental, insect],
  text := [
    trample,
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      (create thatMuch (creatureToken 1 1 [green] [insect]))
  ],
  power := 6,
  toughness := 6
}
