/-- "at the beginning of each <part>" -/
template beginningOfEach (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of each <part>",
  body :=
    beginningOf part none (eachTime none)
}
