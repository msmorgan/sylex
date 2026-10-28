/-- "last turn" -/
template lastTurn : Duration := {
  spelling := "last turn",
  body :=
    durationBetween
      (some
        (beginningOf .turn none (turnRefPrevious none)))
      (some (endOf .turn none (turnRefPrevious none)))
}
