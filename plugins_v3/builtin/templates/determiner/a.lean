/-- "a …" -/
template a (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseA Semantics.ChoiceMode.unmarked) p
}
