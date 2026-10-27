/-- "target …" -/
template target (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: target …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseTarget (Semantics.Templates.Quantity.range (Option.some 1) (Option.some 1))) p
}
