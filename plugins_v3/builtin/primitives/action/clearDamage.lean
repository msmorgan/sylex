/-- Remove marked damage from the subject. -/
primitive clearDamage (subject : NounPhrase) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.clearDamage subject
}
