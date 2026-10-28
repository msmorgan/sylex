/-- "earlier this turn" -/
template earlierThisTurn : Duration := {
  spelling := "earlier this turn",
  body :=
    durationBetween
      (some (beginningOf .turn none current))
      (some now)
}
