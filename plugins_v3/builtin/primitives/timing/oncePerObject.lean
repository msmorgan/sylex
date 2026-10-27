/-- "Activate only once": no more than one while the object exists [CR#602.5b,702.177a]. -/
-- [CR#602.5b,702.177a]
primitive oncePerObject : UsageLimit := {
  spelling := "Activate only once",
  body :=
    Semantics.Primitives.UsageLimit.mk 1 Semantics.Period.thisObject
}
