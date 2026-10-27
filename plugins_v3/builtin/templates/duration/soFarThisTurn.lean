/-- "this turn", as a lookback reads it: from the turn's beginning to now. -/
template soFarThisTurn : Duration := {
  spelling := "this turn",
  body :=
    Semantics.Templates.Duration.durationBetween
      (Option.some (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn Option.none Semantics.Templates.TurnRef.turnRefCurrent))
      (Option.some Semantics.Templates.GameEvent.gameEventNow)
}
