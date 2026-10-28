primitive actionPay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome) (whose : Option NounPhrase) (keyword : Option KeywordLabel) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.pay cost times outcome whose keyword
}
