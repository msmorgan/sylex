primitive attackUnblocked (defender : Option NounPhrase) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.attackUnblocked defender
}
