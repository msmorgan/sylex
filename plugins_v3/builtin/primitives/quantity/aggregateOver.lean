/-- Aggregate the amount evaluated over the given domain. -/
primitive aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.aggregateOver op domain body
}
