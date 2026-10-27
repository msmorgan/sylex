primitive amountTally (clause : Semantics.DidClause) (mode : Semantics.TallyMode) : Semantics.Amount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Amount.tally clause mode
}
