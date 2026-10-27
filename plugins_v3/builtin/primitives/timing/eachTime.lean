/-- "each" occurrence [of <player>'s]: each turn, each upkeep. -/
primitive eachTime (player : Option NounPhrase := none) : TurnRef := {
  -- TODO(spelling): each; wording varies with player, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.TurnRef.each player
}
