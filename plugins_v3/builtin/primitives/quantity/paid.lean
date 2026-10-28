/-- Read the selected facet of a linked payment for the subject. -/
primitive paid (facet : LinkedFacet) (subject : NounPhrase) : Amount := {
  body :=
    Semantics.Primitives.Amount.paid facet subject
}
