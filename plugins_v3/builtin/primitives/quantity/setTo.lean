/-- Set a numeric property to the given amount. -/
primitive setTo (amount : Amount) : (Delta Amount) := {
  body :=
    Semantics.Primitives.Delta.set amount
}
