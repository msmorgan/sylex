/-- "each" occurrence [of <player>'s]: each turn, each upkeep. -/
primitive eachTime (player : Option NounPhrase := none) : TurnRef := {
  -- TODO(spelling): Include or omit the clauses supplied by player.
  body :=
    Semantics.Primitives.TurnRef.each player
}
