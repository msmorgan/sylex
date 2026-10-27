/-- "to pay <cost>" -/
primitive toPay (cost : CostNamed) : SpendPurpose := {
  spelling := "to pay <cost>",
  body :=
    Semantics.Primitives.SpendPurpose.toPay cost
}
