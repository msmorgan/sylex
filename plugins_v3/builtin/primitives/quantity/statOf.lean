/-- Read the specified numeric property of the subject. -/
primitive statOf (axis : ProjAxis) (subject : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.statOf axis subject
}
