/-- "this combat" -/
template thisCombat : Duration := {
  spelling := "this combat",
  body :=
    durationBetween
      (some (beginningOf Semantics.TurnPart.combat none turnRefCurrent))
      (some gameEventNow)
}
