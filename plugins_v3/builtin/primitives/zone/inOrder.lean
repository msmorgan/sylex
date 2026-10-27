/-- "in any order", "in a random order" [CR#401.4] -/
-- [CR#401.4]
primitive inOrder (arrangement : Arrangement) : MoveRider := {
  -- TODO(spelling): in any order; wording varies with arrangement, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.MoveRider.inOrder arrangement
}
