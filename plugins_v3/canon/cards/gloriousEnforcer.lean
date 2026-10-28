card := {
  name := "Glorious Enforcer",
  cost := [5, white, white],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    lifelink,
    at_ (beginningOfPossessed combat eachPlayer)
      (if_ := compareAmt (lifeTotalOf you) greater (lifeTotalOf anOpponent))
      (gain thisCreature doubleStrike untilEndOfTurn)
  ],
  power := 5,
  toughness := 5
}
