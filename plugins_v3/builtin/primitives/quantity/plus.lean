primitive plus (left right : Amount) : Amount := {
  spelling := "<left> plus <right>",
  body :=
    Semantics.Primitives.Amount.arith Semantics.ArithOp.plus left right
}
