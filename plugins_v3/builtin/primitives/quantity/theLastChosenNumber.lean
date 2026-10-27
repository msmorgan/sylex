/-- "the last chosen number" -/
primitive theLastChosenNumber : Amount := {
  spelling := "the last chosen number",
  body :=
    Semantics.Primitives.Amount.chosenNumber Semantics.ChoiceRef.theLatestChoice
}
