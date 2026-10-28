/-- "the total <axis> of <group>": the sum of one projection over a previously named group
("the total power of those creatures"). -/
template totalOf (axis : ProjAxis) (group : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := amountAggregate .sum axis group
}
