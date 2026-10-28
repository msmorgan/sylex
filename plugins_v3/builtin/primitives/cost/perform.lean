/-- A cost paid by performing an action. -/
primitive perform (action : Action) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.perform action
}
