/-- Hold the action until the specified event. -/
primitive holdUntil (action : Action) (event : GameEvent) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.holdUntil action event
}
