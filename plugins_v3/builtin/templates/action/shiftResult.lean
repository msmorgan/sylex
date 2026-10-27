/-- "increase or decrease the result by <amount>" -/
template shiftResult (amount : Amount) : Action := {
  spelling := "increase or decrease the result by <amount>",
  body :=
    Semantics.Templates.Action.actionShiftResult [Semantics.Templates.up amount, Semantics.Templates.down amount]
}
