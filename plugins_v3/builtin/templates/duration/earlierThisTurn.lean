/-- "earlier this turn" -/
template earlierThisTurn : Duration := {
  spelling := "earlier this turn",
  body :=
    durationBetween
      (some (beginningOf Semantics.TurnPart.turn none turnRefCurrent))
      (some gameEventNow)
}
