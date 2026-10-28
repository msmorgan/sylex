/-- "until <player>'s next turn" -/
template untilNextTurnOf (player : NounPhrase) : Duration := {
  spelling := "until <player>'s next turn",
  body :=
    durationBetween none
      (some
        (beginningOf Semantics.TurnPart.turn (some player) (nextTime none)))
}
