/-- "until your next end step" -/
template untilYourNextEndStep : Duration := {
  spelling := "until your next end step",
  body :=
    durationBetween none
      (some
        (beginningOf .endStep (some you)
          (nextTime none)))
}
