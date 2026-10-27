/-- "the chosen color" -/
primitive thatColor : ColorTerm := {
  spelling := "the chosen color",
  body :=
    Semantics.Primitives.ColorTerm.chosen Semantics.ChoiceRef.theChoice
}
