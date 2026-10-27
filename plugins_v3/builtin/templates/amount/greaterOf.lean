/-- "the greater of <a> and <b>", "<a> or <b>, whichever is greater". -/
template greaterOf (a b : Amount) : Amount := {
  spelling := "the greater of <a> and <b>",
  body :=
    Semantics.Templates.Amount.aggregateOf Semantics.AggregateOp.max [a, b]
}
