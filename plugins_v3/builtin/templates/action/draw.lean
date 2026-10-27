/-- "draw <amount> cards" -/
template draw (amount : Amount) : Action := {
  spelling := "draw <amount> cards",
  body :=
    Semantics.Templates.Action.drawCount (Semantics.Templates.Quantity.exactlyOf amount)
}
