/-- "face down", as `spec` lists its characteristics [CR#708.2,406.3a]. -/
-- [CR#708.2,406.3a]
primitive faceDownAs (spec : FaceDown) : MoveRider := {
  spelling := "face down",
  body :=
    Semantics.Primitives.MoveRider.faceDown spec
}
