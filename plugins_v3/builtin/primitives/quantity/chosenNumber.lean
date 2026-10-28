/-- "the chosen number" -/
primitive chosenNumber (ref : ChoiceRef := .theChoice) : Amount := {
  spelling := "the chosen number",
  body := Semantics.Primitives.Amount.chosenNumber ref
}
