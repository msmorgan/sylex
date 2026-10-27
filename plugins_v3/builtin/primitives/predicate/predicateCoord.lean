primitive predicateCoord (coordinator : Semantics.Coordinator) (items : List Semantics.Predicate) : Semantics.Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.coord coordinator items
}
