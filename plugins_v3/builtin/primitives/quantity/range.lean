primitive range (low : Option Nat := none) (high : Option Nat := none) : Quantity := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Quantity.range low high
}
