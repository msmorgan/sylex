primitive amountAggregate (op : AggregateOp) (axis : ProjAxis) (group : NounPhrase) : Amount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Amount.aggregate op axis group
}
