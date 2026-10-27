/-- "the next" occurrence [of <player>'s] -/
primitive nextTime (player : Option NounPhrase := none) : TurnRef := {
  -- TODO(spelling): the next; wording varies with player, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.TurnRef.next player
}
