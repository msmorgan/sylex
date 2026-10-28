/-- "in any order", "in a random order" [CR#401.4] -/
-- [CR#401.4]
primitive inOrder (arrangement : Arrangement) : MoveRider := {
  -- TODO(spelling): The arrangement selects chosen, random, or otherwise specified ordering.
  body :=
    Semantics.Primitives.MoveRider.inOrder arrangement
}
