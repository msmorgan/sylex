/-- "at the beginning of that turn's <part>" -/
template beginningOfThatTurns (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of that turn's <part>",
  body :=
    Semantics.Templates.GameEvent.beginningOf part Option.none Semantics.Templates.TurnRef.thatTime
}
