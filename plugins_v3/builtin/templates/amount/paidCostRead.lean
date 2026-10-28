/-- "<subject>'s <keyword> cost was paid", read back. -/
template paidCostRead (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Amount := {
  -- TODO(spelling): Context-dependent wording: <subject>'s <keyword> cost was paid; string substitution cannot express its omitted or inflected parts.
  body :=
    paid (linkedFacetReadback which window) subject
}
