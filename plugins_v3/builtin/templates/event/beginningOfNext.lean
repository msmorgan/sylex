/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
template beginningOfNext (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of the next <part>",
  body :=
    Semantics.Templates.GameEvent.beginningOf part Option.none (Semantics.Templates.TurnRef.nextTime Option.none)
}
