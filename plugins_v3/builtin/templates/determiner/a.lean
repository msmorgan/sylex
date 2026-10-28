/-- "a …" -/
template a (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a …; string substitution cannot express its omitted or inflected parts.
  body :=
    described (detPhraseA .unmarked) p
}
