/-- "N …", "up to N …" -/
template counted (q : Quantity) (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: N …; string substitution cannot express its omitted or inflected parts.
  body :=
    described (detPhraseCount q none) p
}
