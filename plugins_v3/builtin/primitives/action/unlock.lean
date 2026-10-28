/-- Unlock the specified door. -/
primitive unlock (door : Door) : Action := {
  spelling := "unlock <door>",
  body :=
    Semantics.Primitives.Action.unlock door
}
