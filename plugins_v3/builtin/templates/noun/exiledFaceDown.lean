/-- A card exiled face down: no characteristics [CR#406.3a]. -/
-- [CR#406.3a]
template exiledFaceDown : FaceDown := {
  -- TODO(spelling): A card exiled face down: no characteristics .
  body :=
    { characteristics := Option.none }
}
