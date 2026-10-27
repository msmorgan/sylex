/-- "during <player>'s next turn" (`firstMention` / `laterMention` so `player` is not duplicated
[CR#115.1,608.2c]). -/
-- [CR#115.1,608.2c]
template duringNextTurnOf (player : NounPhrase) : Duration := {
  spelling := "during <player>'s next turn",
  body :=
    Semantics.Templates.Duration.durationBetween
      (Option.some
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn
          (Option.some (player))
          (Semantics.Templates.TurnRef.nextTime Option.none)))
      (Option.some
        (Semantics.Templates.GameEvent.endOf Semantics.TurnPart.turn
          (Option.some (player))
          (Semantics.Templates.TurnRef.nextTime Option.none)))
}
