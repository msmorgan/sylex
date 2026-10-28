/-- "until your next turn" -/
template untilYourNextTurn : Duration := {
  spelling := "until your next turn",
  body :=
    durationBetween none
      (some
        (beginningOf turn (some you)
          (nextTime none)))
}
