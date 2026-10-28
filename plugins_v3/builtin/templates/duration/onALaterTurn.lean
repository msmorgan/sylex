/-- "on a later turn": from the end of this turn on [CR#702.143a,702.170d]. -/
-- [CR#702.143a,702.170d]
template onALaterTurn : Duration := {
  spelling := "on a later turn",
  body :=
    durationBetween
      (some (endOf turn none current)) none
}
