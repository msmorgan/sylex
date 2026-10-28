/-- Apply the arithmetic operator to two amounts. -/
primitive arith (op : ArithOp) (left right : Amount) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.arith op left right
}
