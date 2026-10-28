template may (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := withContinuation .optional body ifDid ifNot
}
