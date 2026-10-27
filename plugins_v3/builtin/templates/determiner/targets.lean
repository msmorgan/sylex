/-- "<q> target <p>s", as "up to two target creatures" or "any number of targets". -/
template targets (q : Quantity) (p : Predicate) : NounPhrase := {
  -- TODO(spelling): <q> target <p>s; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseTarget q) p
}
