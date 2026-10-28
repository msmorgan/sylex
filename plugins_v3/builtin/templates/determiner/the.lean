/-- "the …" -/
template the (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the …; string substitution cannot express its omitted or inflected parts.
  body :=
    described detPhraseThe p
}
