/-- "costs <amount> more" -/
template more (amount : Amount) : CostShift := {
  spelling := "costs <amount> more",
  body :=
    costShiftGeneric (up amount) none
}
