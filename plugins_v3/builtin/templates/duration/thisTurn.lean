/-- "this turn", as an effect lasts: until the turn ends. -/
template thisTurn : Duration := {
  spelling := "this turn",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none
      (Option.some (Semantics.Templates.GameEvent.endOf Semantics.TurnPart.turn Option.none Semantics.Templates.TurnRef.turnRefCurrent))
}
