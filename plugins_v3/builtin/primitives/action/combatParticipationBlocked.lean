primitive combatParticipationBlocked (value : Bool) : Semantics.CombatParticipation := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatParticipation.blocked value
}
