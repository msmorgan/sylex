card := {
  name := "Urza's Filter",
  cost := [4],
  types := [artifact],
  text := [costShift (allOf [isMulticolored, isSpell]) (less 2)]
}
