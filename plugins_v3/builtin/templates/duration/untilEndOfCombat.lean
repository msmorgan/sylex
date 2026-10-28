/-- "until end of combat" -/
template untilEndOfCombat : Duration := {
  spelling := "until end of combat",
  body :=
    durationBetween none
      (some (endOf Semantics.TurnPart.combat none turnRefCurrent))
}
