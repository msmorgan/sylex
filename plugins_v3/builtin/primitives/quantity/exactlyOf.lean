primitive exactlyOf (amount : Amount) : Quantity := {
  spelling := "<amount>",
  body :=
    Semantics.Primitives.Quantity.exactlyOf amount
}
