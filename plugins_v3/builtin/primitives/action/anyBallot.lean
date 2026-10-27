/-- Any of the listed choices, as a vote leaves them open [CR#701.38a]. -/
-- [CR#701.38a]
primitive anyBallot : Ballot := {
  -- TODO(spelling): Any of the listed choices, as a vote leaves them open .
  body :=
    Semantics.Primitives.Ballot.anyBallot
}
