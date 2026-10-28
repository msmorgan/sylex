/-- The condition does not hold. -/
primitive not (condition : Condition) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.not condition
}
