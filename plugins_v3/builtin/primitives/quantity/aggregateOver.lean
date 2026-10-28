/-- Aggregate the amount evaluated over the given domain. -/
primitive aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount) : Amount := {
  body :=
    Semantics.Primitives.Amount.aggregateOver op domain body
}
