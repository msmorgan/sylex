/-- "the number of counters removed this way" -/
primitive removedThisWay : Amount := {
  spelling := "the number of counters removed this way",
  body :=
    Semantics.Primitives.Amount.theOutcome Semantics.OutcomeSort.countersRemoved
}
