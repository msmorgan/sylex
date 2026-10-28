/-- "if <subject>'s <label> cost was paid" -/
template keywordCostPaid (label : KeywordLabel) (subject : NounPhrase) : Condition := {
  spelling := "<subject>'s <label> cost was paid",
  body :=
    costWasPaid (.byKeyword label) none subject
}
