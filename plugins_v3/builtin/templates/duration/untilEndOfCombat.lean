/-- "until end of combat" -/
template untilEndOfCombat : Duration := {
  spelling := "until end of combat",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some (Semantics.Templates.GameEvent.endOf Semantics.TurnPart.combat Option.none Semantics.Templates.TurnRef.turnRefCurrent))
}
