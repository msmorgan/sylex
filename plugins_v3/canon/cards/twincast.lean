/-- Twincast -/
card := {
  name := "Twincast",
  cost := [blue, blue],
  types := [instant],
  text := [spell [copyOnStack (target [isInstantOrSorcery, isSpell]), may (chooseNewTargets (that copy))]]
}
