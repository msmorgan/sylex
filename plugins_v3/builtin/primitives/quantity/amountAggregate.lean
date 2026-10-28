/-- The greatest, least, or total value of the specified numeric property among the group. -/
primitive amountAggregate (op : AggregateOp) (axis : ProjAxis) (group : NounPhrase) : Amount := {
  body := Semantics.Primitives.Amount.aggregate op axis group
}
