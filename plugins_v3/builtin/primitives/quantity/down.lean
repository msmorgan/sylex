/-- Decrease by the given amount. -/
primitive down (amount : Amount) : (Delta Amount) := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Delta.down amount
}
