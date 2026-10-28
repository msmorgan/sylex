/-- "a … of your choice" -/
template aYourChoice (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a … of your choice; string substitution cannot express its omitted or inflected parts.
  body :=
    described (detPhraseA .yourChoice) p
}
