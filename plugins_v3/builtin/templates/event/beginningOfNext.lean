/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
template beginningOfNext (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of the next <part>",
  body :=
    beginningOf part none (nextTime none)
}
