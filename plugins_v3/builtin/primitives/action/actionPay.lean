/-- The actor pays the given cost. -/
primitive actionPay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome) (whose : Option NounPhrase) (keyword : Option KeywordLabel) : Action := {
  body := Semantics.Primitives.Action.pay cost times outcome whose keyword
}
