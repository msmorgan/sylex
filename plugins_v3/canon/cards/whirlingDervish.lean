/-- Whirling Dervish -/
card := {
  name := "Whirling Dervish",
  cost := [green, green],
  types := [creature],
  subtypes := [human, monk],
  text := [
    protection (hasColor black),
    at_ (beginningOfPossessed endStep eachPlayer)
      (if_ := some (did thisCreature (dealDamage anyAmount anOpponent) soFarThisTurn))
      (putCounters 1 p1p1Counter it)
  ],
  power := 1,
  toughness := 1
}
