/-- "You may <body>. [If you do, <ifDid>.] [If you don't, <ifNot>.]" -/
template may (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by ifDid, ifNot.
  body := withContinuation .optional body ifDid ifNot
}
