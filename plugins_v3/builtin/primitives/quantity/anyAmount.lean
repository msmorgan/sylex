/-- "any amount" -/
primitive anyAmount : Amount := {
  spelling := "any amount",
  body :=
    Semantics.Primitives.Amount.anyAmount
}
