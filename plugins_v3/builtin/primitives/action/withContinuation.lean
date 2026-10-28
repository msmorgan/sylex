/-- Perform an action and select its continuation according to the continuation policy. -/
primitive withContinuation (policy : ContinuationPolicy) (body : Action)
    (ifDid : Option Action := none) (ifNot : Option Action := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.withContinuation policy body ifDid ifNot
}
