/-- The event that triggered the enclosing ability [CR#603.2]. -/
-- [CR#603.2]
primitive triggering : LookbackSpan := {
  -- TODO(spelling): The event that triggered the enclosing ability .
  body :=
    Semantics.Primitives.LookbackSpan.triggering
}
