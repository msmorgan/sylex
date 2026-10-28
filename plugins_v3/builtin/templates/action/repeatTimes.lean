/-- "<body> <times> times" -/
template repeatTimes (times : Amount) (body : Action) : Action := {
  spelling := "<body> <times> times",
  body :=
    repeat_ (fixed times body)
}
