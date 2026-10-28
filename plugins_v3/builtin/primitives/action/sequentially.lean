/-- Perform the actions in order. -/
primitive sequentially (steps : List Action := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.sequentially steps
}
