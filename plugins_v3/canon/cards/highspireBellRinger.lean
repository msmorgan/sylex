/-- Highspire Bell-Ringer -/
card := {
  name := "Highspire Bell-Ringer",
  cost := [2, blue],
  types := [creature],
  subtypes := [djinn, monk],
  text := [
    flying,
    costShift (the [isSpell, did you (cast which) thisObject (ordinal := some (nth 2, some turn))])
      (less 1)
  ],
  power := 1,
  toughness := 4
}
