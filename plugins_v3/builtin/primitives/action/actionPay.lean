primitive actionPay (cost : Semantics.Cost) (times : Semantics.PayTimes) (outcome : Semantics.PaymentOutcome) (whose : Option Semantics.NounPhrase) (keyword : Option Semantics.KeywordLabel) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.pay cost times outcome whose keyword
}
