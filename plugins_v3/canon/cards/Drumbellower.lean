card := {
  name := "Drumbellower",
  cost := [2, white],
  types := [creature],
  subtypes := [spirit],
  text := [flying, untapDuring (allOf [isCreature, youControl]) (each isOtherPlayer)],
  power := 2,
  toughness := 1
}
