primitive scaled (cost : Cost) (amount : Amount) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.scaled cost amount
}
