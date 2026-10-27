primitive combatUpdateParticipation (state : Semantics.CombatParticipation) : Semantics.CombatUpdate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatUpdate.participation state
}
