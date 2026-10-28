/-- A card exiled face down: no characteristics [CR#406.3a]. -/
-- [CR#406.3a]
template exiledFaceDown : FaceDown := {
  body :=
    { characteristics := none }
}
