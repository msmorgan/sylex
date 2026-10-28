card := {
  name := "Fluctuator",
  cost := [2],
  types := [artifact],
  text := [
    costShift
      (allOf [abilityHead (keyword Definitions.cycling), did you (activate which) thisObject])
      (less 2)
  ]
}
