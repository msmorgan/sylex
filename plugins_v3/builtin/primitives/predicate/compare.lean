/-- Authoring primitive for `Predicate.compare`. -/
primitive compare (comparator : Comparator) (bound : Amount) (axes : List ProjAxis := []) :
    Predicate := {
  body :=
    Semantics.Primitives.Predicate.compare axes comparator bound
}
