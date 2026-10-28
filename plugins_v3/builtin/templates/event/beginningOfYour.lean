/-- "at the beginning of your <part>", each time it comes -/
template beginningOfYour (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of your <part>",
  body :=
    beginningOf part (some you) (eachTime none)
}
