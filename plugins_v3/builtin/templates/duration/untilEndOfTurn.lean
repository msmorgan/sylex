/-- "until end of turn" -/
template untilEndOfTurn : Duration := {
  spelling := "until end of turn",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some (Semantics.Templates.GameEvent.endOf Semantics.TurnPart.turn Option.none Semantics.Templates.TurnRef.turnRefCurrent))
}
