/-- "if <subject>'s <cost> was paid" -/
template costWasPaid (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Condition := {
  -- TODO(spelling): Include or omit the clauses supplied by window.
  body :=
    compareAmt (paidCostRead which window subject) atLeast 1
}
