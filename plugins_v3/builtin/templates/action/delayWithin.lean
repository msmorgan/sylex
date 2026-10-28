/-- "When <event> <duration>, <action>": a delayed trigger with a window. -/
template delayWithin (event : GameEvent) (duration : Duration) (action : Action)
    (alternatives : List GameEvent := []) : Action := {
  -- TODO(spelling): Format and coordinate the alternatives list entries, including the empty case.
  body := delay event action alternatives (some duration)
}
