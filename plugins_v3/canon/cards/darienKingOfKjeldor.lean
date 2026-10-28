card := {
  name := "Darien, King of Kjeldor",
  cost := [4, white, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    whenever (does any (dealDamage (to := you)))
      (may (create thatMuch (creatureToken 1 1 [white] [soldier])))
  ],
  power := 3,
  toughness := 3
}
