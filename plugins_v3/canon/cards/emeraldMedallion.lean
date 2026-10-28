card := {
  name := "Emerald Medallion",
  cost := [2],
  types := [artifact],
  text := [costShift (allOf [hasColor green, isSpell, did you (cast which) thisObject]) (less 1)]
}
