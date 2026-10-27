/-- A results table: one row per result range [CR#706.3a] -/
-- [CR#706.3a]
primitive applyResultsTable (rows : List RollRow) : Action := {
  -- TODO(spelling): A results table: one row per result range
  body :=
    Semantics.Primitives.Action.applyResultsTable rows
}
