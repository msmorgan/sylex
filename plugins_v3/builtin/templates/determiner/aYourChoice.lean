/-- "a … of your choice" -/
template aYourChoice (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: a … of your choice; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseA Semantics.ChoiceMode.yourChoice) p
}
