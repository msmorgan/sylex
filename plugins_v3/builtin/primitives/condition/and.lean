/-- All of the listed conditions hold. -/
primitive and (conjuncts : List Condition := []) : Condition := {
  body :=
    Semantics.Primitives.Condition.and conjuncts
}
