/-- "until end of turn" -/
template untilEndOfTurn : Duration := {
  spelling := "until end of turn",
  body :=
    durationBetween none
      (some (endOf .turn none current))
}
