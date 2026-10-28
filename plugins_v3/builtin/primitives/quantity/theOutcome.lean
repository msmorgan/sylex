/-- The numeric outcome of the specified kind. -/
primitive theOutcome (sort : OutcomeSort) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.theOutcome sort
}
