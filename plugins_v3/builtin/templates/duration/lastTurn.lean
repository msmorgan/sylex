/-- "last turn" -/
template lastTurn : Duration := {
  spelling := "last turn",
  body :=
    Semantics.Templates.Duration.durationBetween
      (Option.some
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn Option.none (Semantics.Templates.TurnRef.turnRefPrevious Option.none)))
      (Option.some (Semantics.Templates.GameEvent.endOf Semantics.TurnPart.turn Option.none (Semantics.Templates.TurnRef.turnRefPrevious Option.none)))
}
