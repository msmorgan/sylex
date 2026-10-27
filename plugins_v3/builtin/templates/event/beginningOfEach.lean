/-- "at the beginning of each <part>" -/
template beginningOfEach (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of each <part>",
  body :=
    Semantics.Templates.GameEvent.beginningOf part Option.none (Semantics.Templates.TurnRef.eachTime Option.none)
}
