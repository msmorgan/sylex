/-- "enters attacking [<defender>]" -/
primitive entersAttacking (defender : Option NounPhrase) : MoveRider := {
  -- TODO(spelling): Context-dependent wording: enters attacking [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.MoveRider.entersAttacking defender
}
