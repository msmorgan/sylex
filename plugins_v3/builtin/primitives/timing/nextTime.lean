/-- "the next" occurrence [of <player>'s] -/
primitive nextTime (player : Option NounPhrase := none) : TurnRef := {
  -- TODO(spelling): Include or omit the clauses supplied by player.
  body :=
    Semantics.Primitives.TurnRef.next player
}
