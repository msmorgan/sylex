/-- "both halves": a fused split spell [CR#702.102a]. -/
-- [CR#702.102a]
primitive fused : PlayPayment := {
  spelling := "both halves",
  body :=
    Semantics.Primitives.PlayPayment.fused
}
