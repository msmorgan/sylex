/-- "a color", "a creature type": a quality noun. -/
template quality (sort : QualitySort) : Predicate := {
  -- TODO(spelling): a color; wording varies with sort, requiring optional clauses, inflection or structure-dependent phrasing.
  body := qualityNoun sort none
}
