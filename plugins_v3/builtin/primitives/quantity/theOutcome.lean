/-- The numeric outcome of the specified kind. -/
primitive theOutcome (sort : OutcomeSort) : Amount := {
  body :=
    Semantics.Primitives.Amount.theOutcome sort
}
