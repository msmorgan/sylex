/-- "N <p> at random" -/
template countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase := {
  -- TODO(spelling): N <p> at random; wording varies with q, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    described (detPhraseCount q (some Semantics.ChoiceMode.atRandom)) p
}
