/-- Keep the subject from untapping for the specified number of untap steps. -/
primitive skipUntap (subject : NounPhrase) (steps : Amount) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.skipUntap subject steps
}
