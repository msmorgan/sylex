/-- "if <subject>'s <cost> was paid" -/
template costWasPaid (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Condition := {
  -- TODO(spelling): Context-dependent wording: if <subject>'s <cost> was paid; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.paidCostRead which window subject) Semantics.Comparator.atLeast 1
}
