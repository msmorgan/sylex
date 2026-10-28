/-- Authoring primitive for `Predicate.compare`. -/
primitive compare (comparator : Comparator) (bound : Amount) (axes : List ProjAxis := []) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.compare axes comparator bound
}
