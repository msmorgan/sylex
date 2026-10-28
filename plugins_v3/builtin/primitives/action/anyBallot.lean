/-- Any of the listed choices, as a vote leaves them open [CR#701.38a]. -/
-- [CR#701.38a]
primitive anyBallot : Ballot := {
  body :=
    Semantics.Primitives.Ballot.anyBallot
}
