/-- "last turn" -/
template lastTurn : Duration := {
  spelling := "last turn",
  body :=
    durationBetween
      (some
        (beginningOf Semantics.TurnPart.turn none (turnRefPrevious none)))
      (some (endOf Semantics.TurnPart.turn none (turnRefPrevious none)))
}
