/-- A quantity bounded by the given minimum and maximum. -/
primitive range (low : Option Nat := none) (high : Option Nat := none) : Quantity := {
  body :=
    Semantics.Primitives.Quantity.range low high
}
