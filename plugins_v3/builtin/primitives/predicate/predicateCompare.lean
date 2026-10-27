primitive predicateCompare (axes : List Semantics.ProjAxis) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.compare axes comparator bound
}
