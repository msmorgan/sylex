/-- The condition does not hold. -/
primitive not (condition : Condition) : Condition := {
  body :=
    Semantics.Primitives.Condition.not condition
}
