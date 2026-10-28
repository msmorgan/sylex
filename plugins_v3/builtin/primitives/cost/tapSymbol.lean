/-- The tap-symbol cost, {T}. -/
primitive tapSymbol : Cost := {
  spelling := "{T}",
  body :=
    Semantics.Primitives.Cost.tapSymbol
}
