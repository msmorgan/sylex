/-- The event that triggered the enclosing ability [CR#603.2]. -/
-- [CR#603.2]
primitive triggering : LookbackSpan := {
  body :=
    Semantics.Primitives.LookbackSpan.triggering
}
