/-- "the amount by which the ceiling was not reached" -/
primitive shortOfCeiling : Amount := {
  spelling := "the amount by which the ceiling was not reached",
  body :=
    Semantics.Primitives.Amount.theOutcome Semantics.OutcomeSort.ceilingShortfall
}
