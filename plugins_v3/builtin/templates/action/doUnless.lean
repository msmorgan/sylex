/-- "may pay <cost>. If they don't, <action>." -/
template doUnless (action : Action) (cost : Cost) : Action := {
  spelling := "may pay <cost>. If they don't, <action>.",
  body :=
    withContinuation Semantics.ContinuationPolicy.optional
      (actionPay (Semantics.Cost.asPaid cost) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none
        none)
      none (some action)
}
