/-- At least one listed condition holds. -/
primitive or (disjuncts : List Condition := []) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.or disjuncts
}
