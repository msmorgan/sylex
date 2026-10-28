/-- "target …" -/
template target (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: target …; string substitution cannot express its omitted or inflected parts.
  body :=
    described (target (range (some 1) (some 1))) p
}
