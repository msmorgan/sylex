/-- "may pay <cost>. If they don't, <action>." -/
template doUnless (action : Action) (cost : Cost) : Action := {
  spelling := "may pay <cost>. If they don't, <action>.",
  body :=
    Semantics.Templates.Action.withContinuation Semantics.ContinuationPolicy.optional
      (Semantics.Templates.Action.actionPay (Semantics.Cost.asPaid cost) Semantics.PayTimes.once Semantics.PaymentOutcome.paid Option.none
        Option.none)
      Option.none (Option.some action)
}
