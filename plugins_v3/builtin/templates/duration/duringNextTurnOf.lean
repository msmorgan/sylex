/-- "during <player>'s next turn" (`firstMention` / `laterMention` so `player` is not duplicated
[CR#115.1,608.2c]). -/
-- [CR#115.1,608.2c]
template duringNextTurnOf (player : NounPhrase) : Duration := {
  spelling := "during <player>'s next turn",
  body :=
    durationBetween
      (some
        (beginningOf turn
          (some (player))
          (nextTime none)))
      (some
        (endOf turn
          (some (player))
          (nextTime none)))
}
