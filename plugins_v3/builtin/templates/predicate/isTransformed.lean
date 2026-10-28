/-- A transformed permanent is the whole double-faced permanent with its back face up
[CR#701.27g]. -/
-- [CR#701.27g]
template isTransformed : Predicate := {
  -- TODO(spelling): A transformed permanent is the whole double-faced permanent with its back face up .
  body := currentFace .back
}
