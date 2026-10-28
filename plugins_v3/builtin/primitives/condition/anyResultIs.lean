/-- Compare a die result with the specified bound. -/
primitive anyResultIs (comparator : Comparator) (bound : Amount) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.anyResultIs comparator bound
}
