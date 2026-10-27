/-- "that much damage prevented this way" -/
primitive preventedThisWay : Amount := {
  spelling := "that much damage prevented this way",
  body :=
    Semantics.Primitives.Amount.theOutcome Semantics.OutcomeSort.damagePrevented
}
