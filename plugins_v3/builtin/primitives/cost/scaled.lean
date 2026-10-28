/-- A cost repeated the specified number of times. -/
primitive scaled (cost : Cost) (amount : Amount) : Cost := {
  body :=
    Semantics.Primitives.Cost.scaled cost amount
}
