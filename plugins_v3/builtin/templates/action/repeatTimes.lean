/-- "<body> <times> times" -/
template repeatTimes (times : Amount) (body : Action) : Action := {
  spelling := "<body> <times> times",
  body :=
    Semantics.Templates.Action.repeat_ (Semantics.Templates.Repetition.repetitionFixed times body)
}
