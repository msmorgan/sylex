card := {
  name := "Grollub",
  cost := [2, black],
  types := [creature],
  subtypes := [beast],
  text := [
    whenever (does any (dealDamage (to := thisCreature)))
      (instruct eachOpponent (gainLife thatMuch))
  ],
  power := 3,
  toughness := 3
}
