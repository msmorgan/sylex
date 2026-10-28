template keywordCostPaid (label : KeywordLabel) (subject : NounPhrase) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    costWasPaid (.byKeyword label) none subject
}
