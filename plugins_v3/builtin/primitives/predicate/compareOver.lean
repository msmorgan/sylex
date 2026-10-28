/-- Authoring primitive for `Predicate.compareOver`. -/
primitive compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator)
    (bound : Amount) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.compareOver domain measure comparator bound
}
