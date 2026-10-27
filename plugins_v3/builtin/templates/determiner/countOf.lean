/-- "the number of …s": the domain counted is all of them, closed after the count. -/
template countOf (p : Predicate) : Amount := {
  -- TODO(spelling): Context-dependent wording: the number of …s; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Amount.numberOf (Semantics.Templates.NounPhrase.described Semantics.Templates.DetPhrase.detPhraseAll p)
}
