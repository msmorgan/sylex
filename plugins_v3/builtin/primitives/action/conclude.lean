primitive conclude (verb : OutcomeVerb) (cause : Option StateBasedCause := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := Semantics.Primitives.Action.conclude verb cause
}
