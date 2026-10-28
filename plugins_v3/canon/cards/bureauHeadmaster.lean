card := {
  name := "Bureau Headmaster",
  cost := [red, white],
  types := [creature],
  subtypes := [human, assassin],
  text := [
    costShift (allOf [isSpell, equipment, did you (cast which) thisObject]) (less 1),
    costShift (allOf [abilityHead (keyword equip), did you (activate which) thisObject]) (less 1)
  ],
  power := 2,
  toughness := 2
}
