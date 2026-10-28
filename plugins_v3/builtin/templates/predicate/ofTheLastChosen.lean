/-- "of the last chosen <quality>" -/
template ofTheLastChosen (sort : QualitySort) : Predicate := {
  -- TODO(spelling): Context-dependent wording: of the last chosen <quality>; string substitution cannot express its omitted or inflected parts.
  body := ofChosen sort .theLatestChoice
}
