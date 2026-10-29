/-- "may pay <cost>. If they don't, <action>." -/
template doUnless (action : Action) (cost : Cost) : Action := {
  spelling := "may pay <cost>. If they don't, <action>.",
  body :=
    withContinuation .optional
      (actionPay (asPaid cost) once paid none
        none)
      none (some action)
}
