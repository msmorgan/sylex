primitive changeLife (delta : Delta Amount) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.changeLife delta
}
