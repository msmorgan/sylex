primitive coord (coordinator : Coordinator) (items : List Predicate) : Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.coord coordinator items
}
