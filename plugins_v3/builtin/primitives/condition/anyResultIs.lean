/-- Compare a die result with the specified bound. -/
primitive anyResultIs (comparator : Comparator) (bound : Amount) : Condition := {
  body :=
    Semantics.Primitives.Condition.anyResultIs comparator bound
}
