/-- "a color", "a creature type": a quality noun. -/
template quality (sort : QualitySort) : Predicate := {
  spelling := "a <sort>",
  body := qualityNoun sort none
}
