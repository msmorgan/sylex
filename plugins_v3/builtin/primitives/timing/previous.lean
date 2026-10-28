primitive previous (player : Option NounPhrase) : TurnRef := {
  -- TODO(spelling): Include or omit the clauses supplied by player.
  body := Semantics.Primitives.TurnRef.previous player
}
