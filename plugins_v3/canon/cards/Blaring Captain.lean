card := {
  name := "Blaring Captain",
  cost := [3, black],
  types := [creature],
  subtypes := [azra, warrior],
  text := [
    partnerWith (named "Blaring Recruiter"),
    whenever (does thisCreature attack)
      (get (allOf [warrior, isAttacking]) (up 1) (up 1) untilEndOfTurn)
  ],
  power := 2,
  toughness := 2
}
