/-- "monocolored" -/
template isMonocolored : Predicate := {
  spelling := "monocolored",
  body := colorCount .eq 1
}
