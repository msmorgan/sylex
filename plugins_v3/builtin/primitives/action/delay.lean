/-- "When <event>, <action>" as a delayed trigger. -/
primitive delay (event : GameEvent) (action : Action)
    (alternatives : List GameEvent := []) : Action := {
  -- TODO(spelling): When <event>, <action>; wording varies with alternatives, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.delay event alternatives Option.none action
}
