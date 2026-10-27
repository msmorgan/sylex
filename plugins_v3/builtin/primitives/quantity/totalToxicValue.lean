/-- "<subject>'s total toxic value": the sum of the N values of its toxic abilities
[CR#702.164b]. -/
-- [CR#702.164b]
primitive totalToxicValue (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s total toxic value",
  body :=
    Semantics.Primitives.Amount.aggregate Semantics.AggregateOp.sum
      (Semantics.ProjAxis.keywordNumber (Semantics.KeywordLabel.named "toxic")) subject
}
