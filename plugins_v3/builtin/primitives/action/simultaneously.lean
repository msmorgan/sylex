primitive simultaneously (steps : List Action := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.simultaneously steps
}
