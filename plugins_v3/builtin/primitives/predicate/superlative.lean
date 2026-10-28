/-- Authoring primitive for `Predicate.superlative`. -/
primitive superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) :
    Predicate := {
  body :=
    Semantics.Primitives.Predicate.superlative op axis domain
}
