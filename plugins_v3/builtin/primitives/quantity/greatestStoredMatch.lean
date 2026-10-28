/-- The greatest number of matching stored die results on the subject. -/
primitive greatestStoredMatch (subject : NounPhrase) : Amount := {
  body :=
    Semantics.Primitives.Amount.greatestStoredMatch subject
}
