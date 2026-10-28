/-- Authoring primitive for `Predicate.choseExtreme`. -/
primitive choseExtreme (op : AggregateOp) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.choseExtreme op
}
