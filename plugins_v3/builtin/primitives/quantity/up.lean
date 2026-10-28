/-- Increase by the given amount. -/
primitive up (amount : Amount) : (Delta Amount) := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Delta.up amount
}
