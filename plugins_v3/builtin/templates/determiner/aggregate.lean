/-- "the greatest power among …s": the domain ranged over is all of them, closed after it. -/
template aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount := {
  spelling := "the <op> <axis> among <p>",
  body :=
    amountAggregate op axis (described all p)
}
