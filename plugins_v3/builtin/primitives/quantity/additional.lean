/-- "an additional N": beyond what is otherwise allowed ("play an additional land"). -/
primitive additional (q : Quantity) : Quantity := {
  spelling := "an additional <q>",
  body :=
    Semantics.Primitives.Quantity.additional q
}
