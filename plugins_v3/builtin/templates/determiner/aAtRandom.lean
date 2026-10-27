/-- "a … at random" -/
template aAtRandom (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a … at random; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseA Semantics.ChoiceMode.atRandom) p
}
