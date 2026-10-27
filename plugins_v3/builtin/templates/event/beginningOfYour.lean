/-- "at the beginning of your <part>", each time it comes -/
template beginningOfYour (part : TurnPart) : GameEvent := {
  spelling := "at the beginning of your <part>",
  body :=
    Semantics.Templates.GameEvent.beginningOf part (Option.some Semantics.Templates.NounPhrase.you) (Semantics.Templates.TurnRef.eachTime Option.none)
}
