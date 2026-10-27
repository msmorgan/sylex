primitive aggregateOf (op : AggregateOp) (amounts : List Amount := []) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.aggregateOf op amounts
}
