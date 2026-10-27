/-- "this combat" -/
template thisCombat : Duration := {
  spelling := "this combat",
  body :=
    Semantics.Templates.Duration.durationBetween
      (Option.some (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.combat Option.none Semantics.Templates.TurnRef.turnRefCurrent))
      (Option.some Semantics.Templates.GameEvent.gameEventNow)
}
