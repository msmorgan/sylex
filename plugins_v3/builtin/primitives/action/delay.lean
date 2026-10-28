/-- "When <event>, <action>" as a delayed trigger. -/
primitive delay (event : GameEvent) (action : Action) (alternatives : List GameEvent := [])
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): When <event>, <action>; wording varies with alternatives, requiring optional clauses, inflection or structure-dependent phrasing.
  body := Semantics.Primitives.Action.delay event alternatives duration action
}
