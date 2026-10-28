/-- "this turn", as an effect lasts: until the turn ends. -/
template thisTurn : Duration := {
  spelling := "this turn",
  body :=
    durationBetween none
      (some (endOf .turn none turnRefCurrent))
}
