card := {
  name := "Grand Arbiter Augustin IV",
  cost := [2, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, advisor],
  text := [
    costShift (allOf [hasColor white, isSpell, did you (cast which) thisObject]) (less 1),
    costShift (allOf [hasColor blue, isSpell, did you (cast which) thisObject]) (less 1),
    costShift (allOf [isSpell, did yourOpponents (cast which) thisObject]) (more 1)
  ],
  power := 2,
  toughness := 3
}
