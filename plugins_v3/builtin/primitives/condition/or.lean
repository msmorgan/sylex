/-- At least one listed condition holds. -/
primitive or (disjuncts : List Condition := []) : Condition := {
  body :=
    Semantics.Primitives.Condition.or disjuncts
}
