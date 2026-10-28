/-- "<body>. [If you do, <ifDid>.] [If you can't, <ifNot>.]": the body is required. -/
template doIfDone (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := withContinuation .required body ifDid ifNot
}
