template greatestCardsAnOpponentDrew : Amount := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := aggregateOver .max isOpponent (eventCount (draw anyAmount) they soFarThisTurn)
}
