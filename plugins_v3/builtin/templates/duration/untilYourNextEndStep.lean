/-- "until your next end step" -/
template untilYourNextEndStep : Duration := {
  spelling := "until your next end step",
  body :=
    durationBetween none
      (some
        (beginningOf Semantics.TurnPart.endStep (some you)
          (nextTime none)))
}
