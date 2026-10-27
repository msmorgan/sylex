/-- "a color", "a creature type": a quality noun. -/
primitive quality (sort : QualitySort) : Predicate := {
  -- TODO(spelling): a color; wording varies with sort, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Predicate.qualityNoun sort Option.none
}
