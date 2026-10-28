primitive blocking (move : AttachMove) (attacker : NounPhrase) : CombatUpdate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatUpdate.blocking move attacker
}
