/-- A quality noun with its domain: "a color other than blue". -/
template qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate := {
  -- TODO(spelling): The domain selects an enumeration, exclusion, or named-card restriction after
  -- the quality noun.
  body := qualityNoun sort (some domain)
}
