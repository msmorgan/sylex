primitive times (per amount : Amount) : Amount := {
  spelling := "<per> times <amount>",
  body :=
    Semantics.Primitives.Amount.arith Semantics.ArithOp.times per amount
}
