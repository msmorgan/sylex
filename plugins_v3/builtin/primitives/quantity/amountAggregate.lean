primitive amountAggregate (op : Semantics.AggregateOp) (axis : Semantics.ProjAxis) (group : Semantics.NounPhrase) : Semantics.Amount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Amount.aggregate op axis group
}
