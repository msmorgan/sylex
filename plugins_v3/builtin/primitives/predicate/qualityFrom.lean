/-- A quality noun with its domain: "a color other than blue". -/
primitive qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate := {
  -- TODO(spelling): a color other than blue; wording varies with domain, sort, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Predicate.qualityNoun sort (Option.some domain)
}
