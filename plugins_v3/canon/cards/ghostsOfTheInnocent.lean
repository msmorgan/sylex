card := {
  name := "Ghosts of the Innocent",
  cost := [5, white, white],
  types := [creature],
  subtypes := [spirit],
  text := [
    replace (would (a isSource) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (instruct it (dealDamage (half down thatMuch) thatJoin)))
  ],
  power := 4,
  toughness := 5
}
