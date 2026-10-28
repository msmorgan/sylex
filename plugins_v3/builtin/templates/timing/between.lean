/-- A Duration bounded by a starting Event and an ending Event. -/
template between (start end_ : GameEvent) : Duration := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := durationBetween (some start) (some end_)
}
