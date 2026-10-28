card := {
  name := "Garruk's Horde",
  cost := [5, green, green],
  types := [creature],
  subtypes := [beast],
  text := [
    trample,
    visibility reveal you topOfLibrary,
    can you (cast (bare [isSpell, isCreature]) (from_ := onTop))
  ],
  power := 7,
  toughness := 7
}
