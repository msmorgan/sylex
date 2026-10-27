/-- "the chosen number" -/
primitive chosenNumber : Amount := {
  spelling := "the chosen number",
  body :=
    Semantics.Primitives.Amount.chosenNumber Semantics.ChoiceRef.theChoice
}
