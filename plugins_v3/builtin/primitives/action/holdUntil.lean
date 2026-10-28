/-- Hold the action until the specified event. -/
primitive holdUntil (action : Action) (event : GameEvent) : Action := {
  spelling := "<action> until <event>",
  body :=
    Semantics.Primitives.Action.holdUntil action event
}
