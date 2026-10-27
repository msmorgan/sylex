primitive upTo (n : Nat) : Quantity := {
  spelling := "up to <n>",
  body :=
    Semantics.Primitives.Quantity.range Option.none (Option.some n)
}
