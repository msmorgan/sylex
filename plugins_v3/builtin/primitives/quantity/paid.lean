primitive paid (facet : LinkedFacet) (subject : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.paid facet subject
}
