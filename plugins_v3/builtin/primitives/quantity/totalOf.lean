primitive totalOf (axis : ProjAxis) (group : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.aggregate Semantics.AggregateOp.sum axis group
}
