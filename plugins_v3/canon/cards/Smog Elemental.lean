card := {
  name := "Smog Elemental",
  cost := [4, black, black],
  types := [creature],
  subtypes := [elemental],
  text := [flying, getsPt (allOf [isCreature, hasKeyword flying, opponentsControl]) (down 1) (down 1)],
  power := 3,
  toughness := 3
}
