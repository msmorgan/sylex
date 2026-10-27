primitive combatUpdateBlocking (move : Semantics.AttachMove) (attacker : Semantics.NounPhrase) : Semantics.CombatUpdate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatUpdate.blocking move attacker
}
