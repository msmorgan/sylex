/-- A Duration bounded by a starting Event and an ending Event. -/
template between (start end_ : GameEvent) : Duration := {
  spelling := "from <start> until <end_>",
  body := durationBetween (some start) (some end_)
}
