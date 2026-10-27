primitive combatParticipationAttacking (defender : Option Semantics.NounPhrase) : Semantics.CombatParticipation := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatParticipation.attacking defender
}
