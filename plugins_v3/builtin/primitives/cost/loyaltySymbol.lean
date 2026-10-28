/-- A loyalty-symbol cost with the specified change. -/
primitive loyaltySymbol (change : Delta Amount) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.loyaltySymbol change
}
