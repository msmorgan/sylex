/-- "Remove all text found within square brackets" (cleave [CR#702.148a]). -/
-- [CR#702.148a]
primitive removeBracketed : TextChange := {
  spelling := "Remove all text found within square brackets",
  body :=
    Semantics.Primitives.TextChange.removeBracketed
}
