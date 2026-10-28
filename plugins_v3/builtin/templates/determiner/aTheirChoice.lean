/-- "a … of their choice", the chooser being the last-mentioned player. -/
template aTheirChoice (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a … of their choice; string substitution cannot express its omitted or inflected parts.
  body :=
    described (detPhraseA .theirChoice) p
}
