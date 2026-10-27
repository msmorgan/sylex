/-- "When <event> <duration>, <action>": a delayed trigger with a window. -/
primitive delayWithin (event : GameEvent) (duration : Duration) (action : Action)
    (alternatives : List GameEvent := []) : Action := {
  -- TODO(spelling): When <event> <duration>, <action>; wording varies with alternatives, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.delay event alternatives (Option.some duration) action
}
