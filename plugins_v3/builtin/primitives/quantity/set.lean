/-- Set to the given amount. -/
primitive set (amount : Amount) : (Delta Amount) := {
  body :=
    Semantics.Primitives.Delta.set amount
}
