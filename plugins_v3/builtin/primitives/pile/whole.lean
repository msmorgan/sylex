/-- "all of them": a slice that takes the whole group. -/
primitive whole : SliceCount := {
  spelling := "all of them",
  body :=
    Semantics.Primitives.SliceCount.whole
}
