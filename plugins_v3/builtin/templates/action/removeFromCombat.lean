/-- "remove <subject> from combat" -/
template removeFromCombat (subject : NounPhrase) : Action := {
  spelling := "remove <subject> from combat",
  body :=
    Semantics.Templates.Action.combat subject (Semantics.Templates.CombatUpdate.combatUpdateParticipation Semantics.Templates.CombatParticipation.combatParticipationOutsideCombat)
}
