/-- "earlier this turn" -/
template earlierThisTurn : Duration := {
  spelling := "earlier this turn",
  body :=
    Semantics.Templates.Duration.durationBetween
      (Option.some (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn Option.none Semantics.Templates.TurnRef.turnRefCurrent))
      (Option.some Semantics.Templates.GameEvent.gameEventNow)
}
