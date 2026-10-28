/-- Decrease by the given amount. -/
primitive down (amount : Amount) : (Delta Amount) := {
  body :=
    Semantics.Primitives.Delta.down amount
}
