/-- "until your next turn" -/
template untilYourNextTurn : Duration := {
  spelling := "until your next turn",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn (Option.some Semantics.Templates.NounPhrase.you)
          (Semantics.Templates.TurnRef.nextTime Option.none)))
}
