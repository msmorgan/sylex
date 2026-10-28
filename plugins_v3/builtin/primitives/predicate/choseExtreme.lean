/-- Authoring primitive for `Predicate.choseExtreme`. -/
primitive choseExtreme (op : AggregateOp) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.choseExtreme op
}
