/-- "as though its <stat> were <amount> greater" -/
primitive greaterBy (stat : Stat) (amount : Amount) : AsThough := {
  spelling := "as though its <stat> were <amount> greater",
  body :=
    Semantics.Primitives.AsThough.greater stat amount
}
