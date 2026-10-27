/-- Damning Verdict -/
card damningVerdict : Spelled :=
  { name := "Damning Verdict", cost := [generic 3, pip .white, pip .white],
    types := [.sorcery],
    text := [ spell (destroy (allOf (and [isCreature, not hasCounter]))) ] }
