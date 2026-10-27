/-- "on each of your turns", "during each of your turns" -/
template onEachOfYourTurns : Timing := {
  spelling := "on each of your turns",
  body :=
    Semantics.Templates.Timing.timingInPart Semantics.TurnPart.turn (Option.some Semantics.Templates.NounPhrase.you)
      (Semantics.Templates.TurnRef.eachTime Option.none)
}
