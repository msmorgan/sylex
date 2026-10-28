/-- "the greatest power among …s": the domain ranged over is all of them, closed after it. -/
template aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount := {
  -- TODO(spelling): Context-dependent wording: the greatest power among …s; string substitution cannot express its omitted or inflected parts.
  body :=
    amountAggregate op axis (described detPhraseAll p)
}
