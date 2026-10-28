/-- Increase by the given amount. -/
primitive up (amount : Amount) : (Delta Amount) := {
  body :=
    Semantics.Primitives.Delta.up amount
}
