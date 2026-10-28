/-- Authoring primitive for `Predicate.superlative`. -/
primitive superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.superlative op axis domain
}
