/-- "of the last chosen <quality>" -/
primitive ofTheLastChosen (sort : QualitySort) : Predicate := {
  -- TODO(spelling): Context-dependent wording: of the last chosen <quality>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Predicate.ofChosen Semantics.ChoiceRef.theLatestChoice sort
}
