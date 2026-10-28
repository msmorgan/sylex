/-- "draw up to N cards", "can't draw more than one card": a draw bounded by a Quantity. -/
primitive drawCount (quantity : Quantity) : Action := {
  spelling := "draw <quantity> cards",
  body :=
    Semantics.Primitives.Action.draw quantity
}
