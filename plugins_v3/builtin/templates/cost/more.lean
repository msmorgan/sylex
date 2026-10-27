/-- "costs <amount> more" -/
template more (amount : Amount) : CostShift := {
  spelling := "costs <amount> more",
  body :=
    Semantics.Templates.CostShift.costShiftGeneric (Semantics.Templates.up amount) Option.none
}
