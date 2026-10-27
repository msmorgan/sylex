/-- "the last chosen color" -/
primitive theLastChosenColor : ColorTerm := {
  spelling := "the last chosen color",
  body :=
    Semantics.Primitives.ColorTerm.chosen Semantics.ChoiceRef.theLatestChoice
}
