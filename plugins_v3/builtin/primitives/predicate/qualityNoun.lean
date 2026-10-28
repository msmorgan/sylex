/-- Authoring primitive for `Predicate.qualityNoun`. -/
primitive qualityNoun (sort : QualitySort) (domain : Option ChoiceDomain := none) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.qualityNoun sort domain
}
