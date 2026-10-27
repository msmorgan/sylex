/-- A transformed permanent is the whole double-faced permanent with its back face up
[CR#701.27g]. -/
-- [CR#701.27g]
primitive isTransformed : Predicate := {
  -- TODO(spelling): A transformed permanent is the whole double-faced permanent with its back face up .
  body :=
    Semantics.Primitives.Predicate.currentFace Semantics.CardFaceSide.back
}
