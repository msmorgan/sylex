/-- "for <candidates>", as a vote's candidates -/
primitive byCandidate (candidates : NounPhrase) : Ballot := {
  spelling := "for <candidates>",
  body :=
    Semantics.Primitives.Ballot.byCandidate candidates
}
