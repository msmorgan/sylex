card := {
  name := "Shalai and Hallar",
  cost := [1, red, green, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [angel, elf],
  text := [
    flying,
    vigilance,
    whenever (does any (putCounters anyAmount p1p1Counter (a [isCreature, youControl])))
      (instruct thisCreature (dealDamage thatMuch (target isOpponent)))
  ],
  power := 3,
  toughness := 3
}
