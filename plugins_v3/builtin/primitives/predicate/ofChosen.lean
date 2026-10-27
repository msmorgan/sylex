/-- "of the chosen <quality>" -/
primitive ofChosen (sort : QualitySort) : Predicate := {
  -- TODO(spelling): Context-dependent wording: of the chosen <quality>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Predicate.ofChosen Semantics.ChoiceRef.theChoice sort
}
