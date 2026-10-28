/-- Authoring primitive for `Predicate.coord`. -/
primitive coord (coordinator : Coordinator) (items : List Predicate) : Predicate := {
  body := Semantics.Primitives.Predicate.coord coordinator items
}
