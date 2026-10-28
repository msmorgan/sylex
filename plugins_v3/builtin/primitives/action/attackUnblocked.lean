primitive attackUnblocked (defender : Option NounPhrase) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by defender.
  body := Semantics.Primitives.Action.attackUnblocked defender
}
