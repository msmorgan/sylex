/-- "the number of times <subject>'s <keyword> cost was paid" -/
template timesPaid (which : LinkSource) (subject : NounPhrase) : Amount := {
  -- TODO(spelling): Context-dependent wording: the number of times <subject>'s <keyword> cost was paid; string substitution cannot express its omitted or inflected parts.
  body :=
    paid (linkedFacetTimesPaid which) subject
}
