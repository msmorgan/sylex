/-- "<subject> becomes attacking [<defender>]" -/
template becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> becomes attacking [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.combat subject
      (Semantics.Templates.CombatUpdate.combatUpdateParticipation (Semantics.Templates.CombatParticipation.combatParticipationAttacking defender))
}
