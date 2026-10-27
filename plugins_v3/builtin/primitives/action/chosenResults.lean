/-- "ignore <amount> of the results of [<chooser>'s] choice" -/
primitive chosenResults (chooser : Option NounPhrase) (amount : Amount) : IgnoredOutcomes := {
  -- TODO(spelling): Context-dependent wording: ignore <amount> of the results of [<chooser>'s] choice; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.IgnoredOutcomes.chosen chooser amount
}
