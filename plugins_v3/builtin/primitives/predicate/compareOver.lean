/-- Authoring primitive for `Predicate.compareOver`. -/
primitive compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator)
    (bound : Amount) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.compareOver domain measure comparator bound
}
