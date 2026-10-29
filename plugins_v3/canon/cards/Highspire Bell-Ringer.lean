card := {
  name := "Highspire Bell-Ringer",
  cost := [2, blue],
  types := [creature],
  subtypes := [djinn, monk],
  text := [
    flying,
    costShift
      (the
        [isSpell,
          did you (cast which) thisObject (ordinal := some { ordinal := nth 2, per := some turn })])
      (less 1)
  ],
  power := 1,
  toughness := 4
}
