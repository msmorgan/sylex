/-- Apply the arithmetic operator to two amounts. -/
primitive arith (op : ArithOp) (left right : Amount) : Amount := {
  body :=
    Semantics.Primitives.Amount.arith op left right
}
