/-- The player identified by their combat role. -/
primitive combatPlayer (role : CombatRole) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.combatPlayer role
}
