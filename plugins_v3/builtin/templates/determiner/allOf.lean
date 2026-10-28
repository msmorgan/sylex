/-- "all …" -/
template allOf (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: all …; string substitution cannot express its omitted or inflected parts.
  body :=
    described detPhraseAll p
}
