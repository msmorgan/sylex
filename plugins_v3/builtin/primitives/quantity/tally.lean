primitive tally (clause : DidClause) (mode : TallyMode) : Amount := {
  body := Semantics.Primitives.Amount.tally clause mode
}
