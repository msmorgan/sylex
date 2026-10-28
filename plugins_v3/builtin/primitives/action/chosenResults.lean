/-- "ignore <amount> of the results of [<chooser>'s] choice" -/
primitive chosenResults (chooser : Option NounPhrase) (amount : Amount) : IgnoredOutcomes := {
  -- TODO(spelling): Include or omit the clauses supplied by chooser.
  body :=
    Semantics.Primitives.IgnoredOutcomes.chosen chooser amount
}
