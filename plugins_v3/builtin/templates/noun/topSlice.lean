/-- "the top N cards of your library" -/
template topSlice (amount : Amount) : NounPhrase := {
  -- TODO(spelling): the top N cards of your library; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    slice Semantics.PileEnd.top amount (libraryOf you)
}
