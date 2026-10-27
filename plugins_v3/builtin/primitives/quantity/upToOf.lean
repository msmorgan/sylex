primitive upToOf (amount : Amount) : Quantity := {
  spelling := "up to <amount>",
  body :=
    Semantics.Primitives.Quantity.upToOf amount
}
