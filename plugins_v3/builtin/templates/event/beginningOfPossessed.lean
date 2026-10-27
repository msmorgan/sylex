/-- "at the beginning of <possessor>'s <part>", each time it comes -/
template beginningOfPossessed (part : TurnPart) (possessor : NounPhrase) : GameEvent := {
  spelling := "at the beginning of <possessor>'s <part>",
  body :=
    Semantics.Templates.GameEvent.beginningOf part (Option.some possessor) (Semantics.Templates.TurnRef.eachTime Option.none)
}
