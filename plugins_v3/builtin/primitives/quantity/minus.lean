primitive minus (left right : Amount) : Amount := {
  spelling := "<left> minus <right>",
  body :=
    Semantics.Primitives.Amount.arith Semantics.ArithOp.minus left right
}
