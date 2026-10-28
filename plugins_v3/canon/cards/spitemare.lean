card := {
  name := "Spitemare",
  cost := [2, hybridPip red white, hybridPip red white],
  types := [creature],
  subtypes := [elemental],
  text := [
    whenever (does any (dealDamage (to := thisCreature)))
      (instruct it (dealDamage thatMuch (target isAnyTarget)))
  ],
  power := 3,
  toughness := 3
}
