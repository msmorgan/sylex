/-- "<times> more times" -/
primitive moreTimes (times : Amount) : Repetition := {
  spelling := "<times> more times",
  body :=
    Semantics.Primitives.Repetition.moreTimes times
}
