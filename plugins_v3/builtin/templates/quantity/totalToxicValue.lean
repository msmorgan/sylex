/-- "<subject>'s total toxic value": the sum of the N values of its toxic abilities
[CR#702.164b]. -/
-- [CR#702.164b]
template totalToxicValue (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s total toxic value",
  body := amountAggregate .sum (.keywordNumber (.named "toxic")) subject
}
