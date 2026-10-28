/-- The number of votes for the named choice. -/
primitive votesFor (label : String) : Amount := {
  spelling := "the number of votes for <label>",
  body :=
    Semantics.Primitives.Amount.votesFor label
}
