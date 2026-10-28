/-- "this combat" -/
template thisCombat : Duration := {
  spelling := "this combat",
  body :=
    durationBetween
      (some (beginningOf .combat none turnRefCurrent))
      (some gameEventNow)
}
