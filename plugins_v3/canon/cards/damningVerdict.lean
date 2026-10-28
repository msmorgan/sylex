/-- Damning Verdict -/
card := {
  name := "Damning Verdict",
  cost := [3, white, white],
  types := [sorcery],
  text := [spell (destroy (allOf [isCreature, not hasCounter]))]
}
