/-- State the actor's game outcome, optionally identifying its state-based cause. -/
primitive conclude (verb : OutcomeVerb) (cause : Option StateBasedCause := none) : Action := {
  body := Semantics.Primitives.Action.conclude verb cause
}
