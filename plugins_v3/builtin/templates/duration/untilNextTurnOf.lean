/-- "until <player>'s next turn" -/
template untilNextTurnOf (player : NounPhrase) : Duration := {
  spelling := "until <player>'s next turn",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn (Option.some player) (Semantics.Templates.TurnRef.nextTime Option.none)))
}
