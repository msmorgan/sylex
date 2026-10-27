/-- "all …" -/
template allOf (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: all …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described Semantics.Templates.DetPhrase.detPhraseAll p
}
