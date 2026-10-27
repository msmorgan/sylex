primitive oneThrough (n : Nat) : Quantity := {
  spelling := "one to <n>",
  body :=
    Semantics.Primitives.Quantity.range (Option.some 1) (Option.some n)
}
