/-- "N …", "up to N …" -/
template counted (q : Quantity) (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: N …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseCount q Option.none) p
}
