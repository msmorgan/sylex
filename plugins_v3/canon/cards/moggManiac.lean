card := {
  name := "Mogg Maniac",
  cost := [1, red],
  types := [creature],
  subtypes := [goblin],
  text := [
    whenever (does any (dealDamage (to := thisCreature)))
      (instruct it (dealDamage thatMuch (target (or [isPlaneswalker, isOpponent]))))
  ],
  power := 1,
  toughness := 1
}
