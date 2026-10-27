/-- "monocolored" -/
primitive isMonocolored : Predicate := {
  spelling := "monocolored",
  body :=
    Semantics.Primitives.Predicate.colorCount Semantics.Comparator.eq 1
}
