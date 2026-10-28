/-- Update the subject's participation in combat. -/
primitive combat (subject : NounPhrase) (update : CombatUpdate) : Action := {
  body :=
    Semantics.Primitives.Action.combat subject update
}
