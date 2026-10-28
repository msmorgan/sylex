/-- A quality noun with its domain: "a color other than blue". -/
template qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate := {
  -- TODO(spelling): a color other than blue; wording varies with domain, sort, requiring optional clauses, inflection or structure-dependent phrasing.
  body := qualityNoun sort (some domain)
}
