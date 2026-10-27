/-- "until your next end step" -/
template untilYourNextEndStep : Duration := {
  spelling := "until your next end step",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.endStep (Option.some Semantics.Templates.NounPhrase.you)
          (Semantics.Templates.TurnRef.nextTime Option.none)))
}
