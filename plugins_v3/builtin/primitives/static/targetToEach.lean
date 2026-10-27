/-- "Replace all instances of the word 'target' with the word 'each'" (overload
[CR#702.96a]). -/
-- [CR#702.96a]
primitive targetToEach : TextChange := {
  spelling := "Replace all instances of the word 'target' with the word 'each'",
  body :=
    Semantics.Primitives.TextChange.targetToEach
}
