/-- "enters attacking [<defender>]" -/
primitive entersAttacking (defender : Option NounPhrase) : MoveRider := {
  -- TODO(spelling): Include or omit the clauses supplied by defender.
  body :=
    Semantics.Primitives.MoveRider.entersAttacking defender
}
