primitive may (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.withContinuation Semantics.ContinuationPolicy.optional body ifDid ifNot
}
