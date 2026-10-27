primitive orMore (n : Nat) : Quantity := {
  spelling := "<n> or more",
  body :=
    Semantics.Primitives.Quantity.range (Option.some n) Option.none
}
