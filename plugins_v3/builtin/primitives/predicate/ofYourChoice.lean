/-- Authoring primitive for `Predicate.ofYourChoice`. -/
primitive ofYourChoice (sort : QualitySort) (domain : Option ChoiceDomain := none) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.ofYourChoice sort domain
}
