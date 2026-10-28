template totalOf (axis : ProjAxis) (group : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := amountAggregate .sum axis group
}
