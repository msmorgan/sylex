card := {
  name := "Chandra's Spitfire",
  cost := [2, red],
  types := [creature],
  subtypes := [elemental],
  text := [
    flying,
    whenever (does any (dealDamage (to := anOpponent) (kind := some noncombat)))
      (get thisCreature (up 3) (up 0) untilEndOfTurn)
  ],
  power := 1,
  toughness := 3
}
