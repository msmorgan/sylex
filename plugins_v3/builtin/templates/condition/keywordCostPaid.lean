template keywordCostPaid (label : KeywordLabel) (subject : NounPhrase) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Condition.costWasPaid (Semantics.LinkSource.byKeyword label) Option.none subject
}
