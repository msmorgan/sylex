primitive exactly (n : Nat) : Quantity := {
  spelling := "<n>",
  body :=
    Semantics.Primitives.Quantity.range (Option.some n) (Option.some n)
}
