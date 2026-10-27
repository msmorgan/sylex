primitive actionAttackUnblocked (defender : Option Semantics.NounPhrase) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.attackUnblocked defender
}
