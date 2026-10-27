/-- "any number of times" -/
primitive anyNumberOfTimes : Repetition := {
  spelling := "any number of times",
  body :=
    Semantics.Primitives.Repetition.anyNumber
}
