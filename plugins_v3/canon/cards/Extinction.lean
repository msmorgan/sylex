/-- Extinction -/
card := {
  name := "Extinction",
  cost := [4, black],
  types := [sorcery],
  text := [spell (destroy (allOf [isCreature, ofYourChoice (subtype creature)]))]
}
