primitive tally (clause : DidClause) (mode : TallyMode) : Amount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Amount.tally clause mode
}
