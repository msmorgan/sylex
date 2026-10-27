/-- "<subject> wins the game" -/
primitive winGame : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> wins the game; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.conclude Semantics.OutcomeVerb.winGame
}
