card := {
  name := "Daru Warchief",
  cost := [2, white, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    costShift (allOf [soldier, isSpell, did you (cast which) thisObject]) (less 1),
    getsPt (allOf [soldier, isCreature, youControl]) (up 1) (up 2)
  ],
  power := 1,
  toughness := 1
}
