/-- "each …" -/
template each (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: each …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described Semantics.Templates.DetPhrase.detPhraseEach p
}
