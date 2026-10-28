/-- "When <event> <duration>, <action>": a delayed trigger with a window. -/
template delayWithin (event : GameEvent) (duration : Duration) (action : Action)
    (alternatives : List GameEvent := []) : Action := {
  -- TODO(spelling): When <event> <duration>, <action>; wording varies with alternatives, requiring optional clauses, inflection or structure-dependent phrasing.
  body := delay event action alternatives (some duration)
}
