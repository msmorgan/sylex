/-- The greatest number of matching stored die results on the subject. -/
primitive greatestStoredMatch (subject : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.greatestStoredMatch subject
}
