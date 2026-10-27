/-- "the …" -/
template the (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described Semantics.Templates.DetPhrase.detPhraseThe p
}
