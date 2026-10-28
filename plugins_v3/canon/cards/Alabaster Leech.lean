card := {
  name := "Alabaster Leech",
  cost := [white],
  types := [creature],
  subtypes := [leech],
  text := [
    costShift (allOf [isSpell, hasColor white, did you (cast which) thisObject])
      (moreBySymbols (symbols [white]))
  ],
  power := 1,
  toughness := 3
}
