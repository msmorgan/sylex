/-- "<subject>'s <keyword> cost was paid", read back. -/
template paidCostRead (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Amount := {
  -- TODO(spelling): Include or omit the clauses supplied by window.
  body :=
    paid (readback which window) subject
}
