/-- Update the subject's participation in combat. -/
primitive combat (subject : NounPhrase) (update : CombatUpdate) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.combat subject update
}
