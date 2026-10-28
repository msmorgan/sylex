/-- Authoring primitive for `Predicate.qualityNoun`. -/
primitive qualityNoun (sort : QualitySort) (domain : Option ChoiceDomain := none) :
    Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by domain.
  body :=
    Semantics.Primitives.Predicate.qualityNoun sort domain
}
