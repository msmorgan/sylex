/-- "last turn" -/
template lastTurn : Duration := {
  spelling := "last turn",
  body :=
    durationBetween
      (some
        (beginningOf turn none (previous none)))
      (some (endOf turn none (previous none)))
}
